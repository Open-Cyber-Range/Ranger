use super::{ledger::CreateEntry, Ledger};
use crate::services::{
    client::{Deployable, DeploymentInfo},
    deployer::{Deploy, DeployerDistribution},
};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use log::debug;
use ranger_grpc::{capabilities::DeployerTypes, Source as GrpcSource};
use sdl_parser::{
    node::{NodeType, Source as SDLSource},
    Scenario,
};

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
        ledger: &Addr<Ledger>,
    ) -> Result<()>;
}

#[async_trait]
impl DeployableTemplates for Scenario {
    async fn deploy_templates(
        &self,
        deployer_distributor: &Addr<DeployerDistribution>,
        deployers: &[String],
        ledger: &Addr<Ledger>,
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

                    let template_id = deployer_distributor
                        .send(Deploy(
                            DeployerTypes::Template,
                            source.try_to_deployment_command()?,
                            deployers.to_owned(),
                        ))
                        .await??;
                    debug!(
                        "Template {} deployed with template id {}",
                        name, template_id
                    );
                    ledger
                        .send(CreateEntry(Box::new(source.clone()), template_id))
                        .await??;

                    Ok::<()>(())
                }),
        )
        .await?;

        Ok(())
    }
}
