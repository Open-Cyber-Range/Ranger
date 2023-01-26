use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus};
use crate::services::database::account::GetAccount;
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::database::Database;
use crate::services::deployer::{Deploy, DeployerDistribution};
use crate::services::scheduler::{CreateFeatureDeploymentSchedule, Scheduler};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::info;
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{
    Account as GrpcAccount, Feature as GrpcFeature, FeatureResponse,
    FeatureType as GrpcFeatureType, Source as GrpcSource,
};
use sdl_parser::feature::FeatureType;
use sdl_parser::{node::Node, Scenario};

#[async_trait]
pub trait DeployableFeatures {
    async fn deploy_features(&self) -> Result<()>;
}

#[async_trait]
impl DeployableFeatures
    for (
        Addr<DeployerDistribution>,
        Addr<Database>,
        Addr<Scheduler>,
        Vec<String>,
        Scenario,
        Node,
        DeploymentElement,
        Uuid,
        Option<String>,
    )
{
    async fn deploy_features(&self) -> Result<()> {
        let (
            distributor_address,
            database_address,
            scheduler_address,
            deployers,
            scenario,
            node,
            deployment_element,
            exercise_id,
            template_id,
        ) = self;

        let deployment_schedule = scheduler_address
            .send(CreateFeatureDeploymentSchedule(
                scenario.clone(),
                node.clone(),
            ))
            .await??;

        for tranche in deployment_schedule.iter() {
            try_join_all(
                tranche
                    .iter()
                    .map(|(feature_name, feature, username)| async move {
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

                        let mut feature_deployment_element = database_address
                            .send(CreateDeploymentElement(
                                *exercise_id,
                                DeploymentElement::new_ongoing(
                                    deployment_element.deployment_id,
                                    Box::new(feature_name.to_string()),
                                    DeployerTypes::Feature,
                                ),
                            ))
                            .await??;

                        let feature_type = match feature.feature_type.clone() {
                            FeatureType::Service => GrpcFeatureType::Service,
                            FeatureType::Artifact => GrpcFeatureType::Artifact,
                            FeatureType::Configuration => GrpcFeatureType::Configuration,
                        };

                        let template_id = template_id
                            .to_owned()
                            .ok_or_else(|| anyhow!("Template id not found"))?;

                        let account = database_address
                            .send(GetAccount(
                                template_id.as_str().try_into()?,
                                username.to_owned(),
                            ))
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
                            match distributor_address
                                .send(Deploy(
                                    DeployerTypes::Feature,
                                    feature_deployment,
                                    deployers.to_owned(),
                                ))
                                .await?
                            {
                                anyhow::Result::Ok(result) => {
                                    let feature_response = FeatureResponse::try_from(result)?;

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
                                    database_address
                                        .send(UpdateDeploymentElement(
                                            *exercise_id,
                                            feature_deployment_element,
                                        ))
                                        .await??;
                                    Ok(())
                                }
                                Err(err) => {
                                    feature_deployment_element.status = ElementStatus::Failed;
                                    database_address
                                        .send(UpdateDeploymentElement(
                                            *exercise_id,
                                            feature_deployment_element,
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
