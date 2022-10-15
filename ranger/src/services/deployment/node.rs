use super::ledger::{CreateEntry, GetEntry};
use super::Ledger;
use crate::models::Deployment;
use crate::services::client::{Deployable, DeploymentInfo};
use crate::services::deployer::{Deploy, DeployerDistribution};
use crate::services::scheduler::{CreateDeploymentSchedule, Scheduler};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use ranger_grpc::capabilities::DeployerTypes;
use ranger_grpc::{
    Configuration, DeploySwitch, DeployVirtualMachine, MetaInfo, Switch, VirtualMachine,
};
use sdl_parser::node::{Node, NodeType, Source};
use sdl_parser::Scenario;

impl Deployable
    for (
        String,
        Node,
        Deployment,
        String,
        Vec<String>,
        Option<String>,
    )
{
    fn try_to_deployment_command(&self) -> Result<Box<dyn DeploymentInfo>> {
        let (name, node, deployment, exercise_name, links, template_id) = self;
        let meta_info = Some(MetaInfo {
            exercise_name: exercise_name.to_string(),
            deployment_name: deployment.name.to_string(),
        });
        let name = name.to_string();
        let links = links.to_vec();
        Ok(match node.type_field {
            NodeType::VM => Box::new(DeployVirtualMachine {
                virtual_machine: Some(VirtualMachine {
                    name,
                    links,
                    configuration: node.resources.as_ref().map(|resource| Configuration {
                        cpu: resource.cpu,
                        ram: resource.ram,
                    }),
                    template_id: template_id
                        .clone()
                        .ok_or_else(|| anyhow!("Template not found"))?,
                }),
                meta_info,
            }),
            NodeType::Switch => Box::new(DeploySwitch {
                switch: Some(Switch { name, links }),
                meta_info,
            }),
        })
    }
}

#[async_trait]
pub trait DeployableNodes {
    async fn deploy_nodes<'a>(
        &'a self,
        distributor_address: &'a Addr<DeployerDistribution>,
        scheduler_address: &'a Addr<Scheduler>,
        ledger_address: &'a Addr<Ledger>,
        exercise_name: &'a str,
        deployment: &'a Deployment,
        deployers: &'a [String],
    ) -> Result<()>;
}

async fn get_template_id(source: &Option<Source>, ledger: &Addr<Ledger>) -> Result<Option<String>> {
    if let Some(source) = source {
        let template_id = ledger.send(GetEntry(Box::new(source.to_owned()))).await??;

        return Ok(Some(template_id));
    }
    Ok(None)
}

#[async_trait]
impl DeployableNodes for Scenario {
    async fn deploy_nodes<'a>(
        &'a self,
        distributor_address: &'a Addr<DeployerDistribution>,
        scheduler_address: &'a Addr<Scheduler>,
        ledger_address: &'a Addr<Ledger>,
        exercise_name: &'a str,
        deployment: &'a Deployment,
        deployers: &'a [String],
    ) -> Result<()> {
        let deployment_schedule = scheduler_address
            .send(CreateDeploymentSchedule(self.clone()))
            .await??;
        for tranche in deployment_schedule.iter() {
            try_join_all(
                tranche
                    .iter()
                    .map(|(unique_name, node, infra_node)| async move {
                        let links =
                            try_join_all(infra_node.links.clone().unwrap_or_default().iter().map(
                                |link_name| async move {
                                    ledger_address
                                        .send(GetEntry(Box::new(link_name.to_string())))
                                        .await?
                                },
                            ))
                            .await?;
                        let template_id = get_template_id(&node.source, ledger_address).await?;
                        let command = (
                            unique_name.clone(),
                            node.clone(),
                            deployment.to_owned(),
                            exercise_name.to_string(),
                            links,
                            template_id,
                        )
                            .try_to_deployment_command()?;
                        let id = distributor_address
                            .send(Deploy(
                                match node.type_field {
                                    sdl_parser::node::NodeType::VM => DeployerTypes::VirtualMachine,
                                    sdl_parser::node::NodeType::Switch => DeployerTypes::Switch,
                                },
                                command,
                                deployers.to_owned(),
                            ))
                            .await??;
                        ledger_address
                            .send(CreateEntry(Box::new(unique_name.to_string()), id))
                            .await??;
                        Ok(())
                    })
                    .collect::<Vec<_>>(),
            )
            .await?;
        }
        Ok(())
    }
}