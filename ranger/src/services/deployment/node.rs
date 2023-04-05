use super::condition::DeployableConditions;
use super::feature::DeployableFeatures;
use crate::models::helpers::{deployer_type::DeployerType, uuid::Uuid};
use crate::models::{Deployment, DeploymentElement, ElementStatus, Exercise};
use crate::services::client::{Deployable, DeploymentInfo};
use crate::services::database::deployment::{
    CreateDeploymentElement, GetDeploymentElementByDeploymentIdByScenarioReference,
    UpdateDeploymentElement,
};
use crate::services::database::Database;
use crate::services::deployer::{Deploy, DeployerDistribution, UnDeploy};
use crate::services::scheduler::{CreateDeploymentSchedule, Scheduler};
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use ranger_grpc::{
    capabilities::DeployerTypes, Configuration, DeploySwitch, DeployVirtualMachine, Identifier,
    MetaInfo, Switch, VirtualMachine,
};
use sdl_parser::{
    common::Source,
    node::{Node, NodeType},
    Scenario,
};

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
    async fn deploy_nodes(
        &self,
        distributor_address: &Addr<DeployerDistribution>,
        scheduler_address: &Addr<Scheduler>,
        database_address: &Addr<Database>,
        exercise: &Exercise,
        deployment: &Deployment,
        deployers: &[String],
    ) -> Result<()>;
}

async fn get_template_id(
    deployment_id: Uuid,
    source: &Option<Source>,
    database_address: &Addr<Database>,
) -> Result<Option<String>> {
    if let Some(source) = source {
        let deployment_element = database_address
            .send(GetDeploymentElementByDeploymentIdByScenarioReference(
                deployment_id,
                Box::new(source.to_owned()),
                true,
            ))
            .await??;

        return Ok(deployment_element.handler_reference);
    }
    Ok(None)
}

#[async_trait]
impl DeployableNodes for Scenario {
    async fn deploy_nodes(
        &self,
        distributor_address: &Addr<DeployerDistribution>,
        scheduler_address: &Addr<Scheduler>,
        database_address: &Addr<Database>,
        exercise: &Exercise,
        deployment: &Deployment,
        deployers: &[String],
    ) -> Result<()> {
        let deployment_schedule = scheduler_address
            .send(CreateDeploymentSchedule(self.clone()))
            .await??;
        for tranche in deployment_schedule.iter() {
            try_join_all(
                tranche
                    .iter()
                    .map(|(unique_name, node, infra_node)| async move {
                        let deployer_type = match node.type_field {
                            sdl_parser::node::NodeType::VM => DeployerTypes::VirtualMachine,
                            sdl_parser::node::NodeType::Switch => DeployerTypes::Switch,
                        };
                        let mut deployment_element = database_address
                            .send(CreateDeploymentElement(
                                exercise.id,
                                DeploymentElement::new_ongoing(
                                    deployment.id,
                                    Box::new(unique_name.to_string()),
                                    deployer_type,
                                ),
                                true,
                            ))
                            .await??;
                        let links =
                            try_join_all(infra_node.links.clone().unwrap_or_default().iter().map(
                                |link_name| async move {
                                    let deployment_element = database_address
                                        .send(
                                            GetDeploymentElementByDeploymentIdByScenarioReference(
                                                deployment.id,
                                                Box::new(link_name.to_string()),
                                                true,
                                            ),
                                        )
                                        .await??;
                                    deployment_element
                                        .handler_reference
                                        .ok_or_else(|| anyhow!("Handler reference not found"))
                                },
                            ))
                            .await?;
                        let template_id =
                            get_template_id(deployment.id, &node.source, database_address).await?;
                        let command = (
                            unique_name.clone(),
                            node.clone(),
                            deployment.to_owned(),
                            exercise.name.to_string(),
                            links,
                            template_id.clone(),
                        )
                            .try_to_deployment_command()?;

                        match distributor_address
                            .send(Deploy(deployer_type, command, deployers.to_owned()))
                            .await?
                        {
                            anyhow::Result::Ok(client_response) => {
                                let id = Identifier::try_from(client_response)?.value;

                                deployment_element.status = ElementStatus::Success;
                                deployment_element.handler_reference = Some(id);
                                database_address
                                    .send(UpdateDeploymentElement(
                                        exercise.id,
                                        deployment_element.clone(),
                                        true,
                                    ))
                                    .await??;

                                if node.type_field == NodeType::VM {
                                    if node.features.is_some() {
                                        (
                                            distributor_address.clone(),
                                            database_address.clone(),
                                            scheduler_address.clone(),
                                            deployers.to_owned(),
                                            self.clone(),
                                            node.clone(),
                                            deployment_element.clone(),
                                            exercise.id,
                                            template_id.clone(),
                                        )
                                            .deploy_features()
                                            .await?;
                                    }
                                    if node.conditions.is_some() {
                                        (
                                            distributor_address.clone(),
                                            database_address.clone(),
                                            deployers.to_owned(),
                                            self.clone(),
                                            node.clone(),
                                            deployment_element,
                                            exercise.id,
                                            template_id,
                                        )
                                            .deploy_conditions()
                                            .await?;
                                    }
                                };

                                Ok(())
                            }
                            Err(error) => {
                                deployment_element.status = ElementStatus::Failed;
                                database_address
                                    .send(UpdateDeploymentElement(
                                        exercise.id,
                                        deployment_element,
                                        true,
                                    ))
                                    .await??;
                                Err(error)
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

#[async_trait]
pub trait RemoveableNodes {
    async fn undeploy_nodes<'a>(
        &'a self,
        distributor_address: &'a Addr<DeployerDistribution>,
        database_address: &'a Addr<Database>,
        deployers: &'a [String],
        exercise_id: &'a Uuid,
    ) -> Result<()>;
}

#[async_trait]
impl RemoveableNodes for Vec<DeploymentElement> {
    async fn undeploy_nodes<'a>(
        &'a self,
        distributor_address: &'a Addr<DeployerDistribution>,
        database_address: &'a Addr<Database>,
        deployers: &'a [String],
        exercise_id: &'a Uuid,
    ) -> Result<()> {
        try_join_all(self.iter().map(|element| async move {
            match element.deployer_type {
                DeployerType(DeployerTypes::VirtualMachine | DeployerTypes::Switch) => {
                    if let Some(handler_reference) = &element.handler_reference {
                        let mut element_update = element.clone();

                        return match distributor_address
                            .send(UnDeploy(
                                element.deployer_type.0,
                                handler_reference.to_string(),
                                deployers.to_owned(),
                            ))
                            .await?
                        {
                            anyhow::Result::Ok(_) => {
                                element_update.status = ElementStatus::Removed;
                                database_address
                                    .send(UpdateDeploymentElement(
                                        exercise_id.to_owned(),
                                        element_update,
                                        true,
                                    ))
                                    .await??;
                                Ok(())
                            }
                            Err(error) => {
                                element_update.status = ElementStatus::RemoveFailed;
                                database_address
                                    .send(UpdateDeploymentElement(
                                        exercise_id.to_owned(),
                                        element_update,
                                        true,
                                    ))
                                    .await??;
                                Err(error)
                            }
                        };
                    }
                    Err(anyhow!("Handler reference not found"))
                }
                _ => Ok(()),
            }
        }))
        .await?;
        Ok(())
    }
}
