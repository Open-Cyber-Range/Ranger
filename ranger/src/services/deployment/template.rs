use crate::{
    models::{Deployment, DeploymentElement, ElementStatus, Exercise},
    services::{
        client::{Deployable, DeploymentInfo},
        database::{
            deployment::{CreateDeploymentElement, UpdateDeploymentElement},
            Database,
        },
        deployer::{Deploy, DeployerDistribution},
    },
};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::debug;
use ranger_grpc::{capabilities::DeployerTypes, Identifier, Source as GrpcSource};
use sdl_parser::{common::Source as SDLSource, node::NodeType, Scenario};

impl Deployable for SDLSource {
    fn try_to_deployment_command(&self) -> Result<Box<dyn DeploymentInfo>> {
        Ok(Box::new(GrpcSource {
            name: self.name.clone(),
            version: self.version.clone(),
        }))
    }
}

#[async_trait]
pub trait DeployableTemplates {
    async fn deploy_templates(
        &self,
        deployer_distributor: &Addr<DeployerDistribution>,
        deployers: &[String],
        database_address: &Addr<Database>,
        deployment: &Deployment,
        exercise: &Exercise,
    ) -> Result<()>;
}

#[async_trait]
impl DeployableTemplates for Scenario {
    async fn deploy_templates(
        &self,
        deployer_distributor: &Addr<DeployerDistribution>,
        deployers: &[String],
        database_address: &Addr<Database>,
        deployment: &Deployment,
        exercise: &Exercise,
    ) -> Result<()> {
        let nodes = self
            .nodes
            .as_ref()
            .ok_or_else(|| anyhow!("Nodes not found"))?;
        try_join_all(
            nodes
                .iter()
                .filter(|(_, node)| node.type_field == NodeType::VM)
                .map(|(name, node)| async move {
                    let source = node
                        .source
                        .as_ref()
                        .ok_or_else(|| anyhow!("Source not found"))?;

                    let mut deployment_element = database_address
                        .send(CreateDeploymentElement(
                            exercise.id,
                            DeploymentElement::new(
                                deployment.id,
                                Box::new(source.to_owned()),
                                DeployerTypes::Template,
                            ),
                        ))
                        .await??;

                    match deployer_distributor
                        .send(Deploy(
                            DeployerTypes::Template,
                            source.try_to_deployment_command()?,
                            deployers.to_owned(),
                        ))
                        .await?
                    {
                        anyhow::Result::Ok(client_response) => {
                            let template_id = Identifier::try_from(client_response)?.value;

                            debug!(
                                "Template {} deployed with template id {}",
                                name, template_id
                            );
                            deployment_element.status = ElementStatus::Success;
                            deployment_element.handler_reference = Some(template_id);

                            database_address
                                .send(UpdateDeploymentElement(exercise.id, deployment_element))
                                .await??;
                            Ok::<()>(())
                        }
                        Err(error) => {
                            deployment_element.status = ElementStatus::Failed;
                            database_address
                                .send(UpdateDeploymentElement(exercise.id, deployment_element))
                                .await??;
                            Err(error)
                        }
                    }
                }),
        )
        .await?;

        Ok(())
    }
}
