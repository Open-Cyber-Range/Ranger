use crate::models::helpers::uuid::Uuid;
use crate::models::{DeploymentElement, ElementStatus};
use crate::services::database::deployment::{CreateDeploymentElement, UpdateDeploymentElement};
use crate::services::database::Database;
use crate::services::deployer::{Deploy, DeployerDistribution};
use crate::services::scheduler::{CreateFeatureDeploymentSchedule, Scheduler};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{Feature as GrpcFeature, FeatureType as GrpcFeatureType, Source as GrpcSource};
use sdl_parser::feature::FeatureType;
use sdl_parser::{node::Node, Scenario};

#[async_trait]
pub trait DeployableFeatures {
    async fn deploy_features<'a>(&'a self) -> Result<()>;
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
    async fn deploy_features<'a>(&'a self) -> Result<()> {
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

        // returns a schedule for ALL features in the scenario
        // let deployment_schedule = scheduler_address
        //     .send(CreateFeatureDeploymentSchedule(
        //         scenario.clone(),
        //         node.clone(),
        //     ))
        //     .await??;

        //deploys (and executes) all of the nodes features in parallel
        if let Some(node_features) = node.features.clone() {
            try_join_all(
                node_features
                    .iter()
                    .map(|(feature_name, role)| async move {
                        let features = scenario
                            .features
                            .clone()
                            .ok_or_else(|| anyhow!("scenario features"))?;

                        let feature = features
                            .get(feature_name)
                            .ok_or_else(|| anyhow!("feature missing in features"))?;

                        let virtual_machine_id = deployment_element
                            .handler_reference
                            .clone()
                            .ok_or_else(|| anyhow!("deployment element handler reference"))?;

                        let feature_source = feature
                            .source
                            .clone()
                            .ok_or_else(|| anyhow!("feature source"))?;

                        let mut feature_deployment_element = database_address
                            .send(CreateDeploymentElement(
                                *exercise_id,
                                DeploymentElement::new(
                                    deployment_element.deployment_id,
                                    Box::new(feature_name.to_string()),
                                    DeployerTypes::Feature,
                                ),
                            ))
                            .await??;

                        //throw this into a From<FeatureType> ?
                        //both structs are external and it would necessitate a helper struct
                        let feature_type = match feature.feature_type.clone() {
                            FeatureType::Service => GrpcFeatureType::Service,
                            FeatureType::Artifact => GrpcFeatureType::Artifact,
                            FeatureType::Configuration => GrpcFeatureType::Configuration,
                        };
                        let username = node
                            .clone()
                            .roles
                            .ok_or_else(|| anyhow!("node roles"))?
                            .get(role)
                            .ok_or_else(|| anyhow!("user role"))?
                            .to_owned();

                        let feature_depoyment = Box::new(GrpcFeature {
                            name: feature_name.to_owned(),
                            virtual_machine_id,
                            username,
                            feature_type: feature_type.into(),
                            template_id: template_id
                                .clone()
                                .ok_or_else(|| anyhow!("template id for feature credentials"))?,
                            source: Some(GrpcSource {
                                name: feature_source.name,
                                version: feature_source.version,
                            }),
                        });

                        {
                            match distributor_address
                                .send(Deploy(
                                    DeployerTypes::Feature,
                                    feature_depoyment,
                                    deployers.to_owned(),
                                ))
                                .await?
                            {
                                anyhow::Result::Ok(id) => {
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
