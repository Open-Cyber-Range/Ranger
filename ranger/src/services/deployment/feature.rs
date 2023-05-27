use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus, Exercise};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::deployer::Deploy;
use crate::services::scheduler::CreateFeatureDeploymentSchedule;
use crate::Addressor;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::info;
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{
    Account as GrpcAccount, ExecutorResponse, Feature as GrpcFeature,
    FeatureType as GrpcFeatureType, Source as GrpcSource,
};
use sdl_parser::feature::FeatureType;
use sdl_parser::node::NodeType;
use sdl_parser::{node::Node, Scenario};

#[async_trait]
pub trait DeployableFeatures {
    async fn deploy_scenario_features(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(Node, DeploymentElement, Uuid)],
    ) -> Result<()>;
}
#[async_trait]
impl DeployableFeatures for Scenario {
    async fn deploy_scenario_features(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployers: &[String],
        deployed_nodes: &[(Node, DeploymentElement, Uuid)],
    ) -> Result<()> {
        if self.features.is_some() {
            try_join_all(deployed_nodes.iter().map(
                |(node, deployment_element, template_id)| async move {
                    if matches!(node.type_field, NodeType::VM) {
                        (
                            addressor.clone(),
                            deployers.to_owned(),
                            self.clone(),
                            node.clone(),
                            deployment_element.clone(),
                            exercise.id,
                            *template_id,
                        )
                            .deploy_node_features()
                            .await?;
                    }
                    Ok(())
                },
            ))
            .await?;
        }
        Ok(())
    }
}

#[async_trait]
pub trait DeployableNodeFeatures {
    async fn deploy_node_features(&self) -> Result<()>;
}

#[async_trait]
impl DeployableNodeFeatures
    for (
        Addressor,
        Vec<String>,
        Scenario,
        Node,
        DeploymentElement,
        Uuid,
        Uuid,
    )
{
    async fn deploy_node_features(&self) -> Result<()> {
        let (addressor, deployers, scenario, node, deployment_element, exercise_id, template_id) =
            self;

        let deployment_schedule = addressor
            .scheduler
            .send(CreateFeatureDeploymentSchedule(
                scenario.clone(),
                node.clone(),
            ))
            .await??;

        for tranche in deployment_schedule.iter() {
            try_join_all(
                tranche
                    .iter()
                    .map(|(feature_name, feature, role)| async move {
                        info!(
                            "Deploying feature '{feature_name}' for VM {node_name}",
                            node_name = deployment_element.scenario_reference
                        );

                        let virtual_machine_id = deployment_element
                            .handler_reference
                            .clone()
                            .ok_or_else(|| {
                                anyhow!("Deployment element handler reference not found")
                            })?;

                        let feature_source = feature
                            .source
                            .clone()
                            .ok_or_else(|| anyhow!("Feature source not found"))?;

                        let mut feature_deployment_element = addressor
                            .database
                            .send(CreateDeploymentElement(
                                *exercise_id,
                                DeploymentElement::new_ongoing(
                                    deployment_element.deployment_id,
                                    Box::new(feature_name.to_string()),
                                    DeployerTypes::Feature,
                                ),
                                true,
                            ))
                            .await??;

                        let feature_type = match feature.feature_type.clone() {
                            FeatureType::Service => GrpcFeatureType::Service,
                            FeatureType::Artifact => GrpcFeatureType::Artifact,
                            FeatureType::Configuration => GrpcFeatureType::Configuration,
                        };

                        let account = addressor
                            .database
                            .send(GetAccount(*template_id, role.username.to_owned()))
                            .await??;

                        let feature_deployment = Box::new(GrpcFeature {
                            name: feature_name.to_owned(),
                            virtual_machine_id,
                            feature_type: feature_type.into(),
                            account: Some(GrpcAccount {
                                username: account.username,
                                password: account.password.unwrap_or_default(),
                                private_key: account.private_key.unwrap_or_default(),
                            }),
                            source: Some(GrpcSource {
                                name: feature_source.name,
                                version: feature_source.version,
                            }),
                        });

                        {
                            match addressor
                                .distributor
                                .send(Deploy(
                                    DeployerTypes::Feature,
                                    feature_deployment,
                                    deployers.to_owned(),
                                ))
                                .await?
                            {
                                anyhow::Result::Ok(result) => {
                                    let feature_response = ExecutorResponse::try_from(result)?;

                                    let id = feature_response
                                        .identifier
                                        .ok_or_else(|| {
                                            anyhow!("Identifier in Feature Response not found")
                                        })?
                                        .value;

                                    if feature_type == GrpcFeatureType::Service {
                                        info!(
                                            "Feature: '{feature_name}' output: {:?}",
                                            feature_response.vm_log
                                        );
                                        feature_deployment_element.executor_log =
                                            Some(feature_response.vm_log);
                                    }

                                    feature_deployment_element.status = ElementStatus::Success;
                                    feature_deployment_element.handler_reference = Some(id);
                                    addressor
                                        .database
                                        .send(UpdateDeploymentElement(
                                            *exercise_id,
                                            feature_deployment_element,
                                            true,
                                        ))
                                        .await??;
                                    Ok(())
                                }
                                Err(err) => {
                                    feature_deployment_element.status = ElementStatus::Failed;
                                    addressor
                                        .database
                                        .send(UpdateDeploymentElement(
                                            *exercise_id,
                                            feature_deployment_element,
                                            true,
                                        ))
                                        .await??;
                                    Err(err)
                                }
                            }
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .await?;
        }
        Ok(())
    }
}
