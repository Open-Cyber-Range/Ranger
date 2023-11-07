use crate::models::helpers::{deployer_type::DeployerType, uuid::Uuid};
use crate::models::{Deployment, DeploymentElement, ElementStatus, Exercise};
use crate::services::client::{Deployable, DeploymentInfo};
use crate::services::database::deployment::{
    CreateDeploymentElement, GetDeploymentElementByDeploymentIdByScenarioReference,
    UpdateDeploymentElement,
};
use crate::services::database::Database;
use crate::services::deployer::{Deploy, UnDeploy};
use crate::services::scheduler::CreateDeploymentSchedule;
use crate::utilities::try_some;
use crate::Addressor;
use actix::Addr;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::future::try_join_all;
use ranger_grpc::{
    capabilities::DeployerType as GrpcDeployerType, Configuration, DeploySwitch, DeployVirtualMachine, Identifier,
    MetaInfo, Switch, VirtualMachine,
};
use sdl_parser::{
    common::Source,
    node::{Node, NodeType},
    Scenario,
};

impl Deployable for (String, Node, Deployment, String, Vec<String>, Option<Uuid>) {
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
                    template_id: try_some(template_id.to_owned(), "Template Id not found")?
                        .to_string(),
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
        addressor: &Addressor,
        exercise: &Exercise,
        deployment: &Deployment,
        deployers: &[String],
    ) -> Result<Vec<(Node, DeploymentElement, Uuid)>>;
}

async fn get_template_id(
    deployment_id: Uuid,
    node_source: &Option<Source>,
    database_address: &Addr<Database>,
) -> Result<Option<Uuid>> {
    if let Some(source) = node_source.clone() {
        let deployment_element = database_address
            .send(GetDeploymentElementByDeploymentIdByScenarioReference(
                deployment_id,
                Box::new(source.to_owned()),
                true,
            ))
            .await??;
        let template_id_string = try_some(
            deployment_element.handler_reference,
            "Error getting Template Id: Node Deployment Element missing Handler Reference",
        )?;
        let template_id: Uuid = template_id_string.as_str().try_into()?;

        return Ok(Some(template_id));
    }
    Ok(None)
}

#[async_trait]
impl DeployableNodes for Scenario {
    async fn deploy_nodes(
        &self,
        addressor: &Addressor,
        exercise: &Exercise,
        deployment: &Deployment,
        deployers: &[String],
    ) -> Result<Vec<(Node, DeploymentElement, Uuid)>> {
        let deployment_schedule = addressor
            .scheduler
            .send(CreateDeploymentSchedule(self.clone()))
            .await??;

        let mut deployment_results = vec![];
        for tranche in deployment_schedule.iter() {
            let tranche_results = try_join_all(
                tranche
                    .iter()
                    .map(|(unique_name, node, infra_node)| async move {
                        let deployer_type = match node.type_field {
                            sdl_parser::node::NodeType::VM => GrpcDeployerType::VirtualMachine,
                            sdl_parser::node::NodeType::Switch => GrpcDeployerType::Switch,
                        };
                        let mut deployment_element = addressor
                            .database
                            .send(CreateDeploymentElement(
                                exercise.id,
                                DeploymentElement::new_ongoing(
                                    deployment.id,
                                    Box::new(unique_name.to_string()),
                                    deployer_type,
                                    None,
                                    None,
                                ),
                                true,
                            ))
                            .await??;
                        let links =
                            try_join_all(infra_node.links.clone().unwrap_or_default().iter().map(
                                |link_name| async move {
                                    let deployment_element = addressor
                                        .database
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
                            get_template_id(deployment.id, &node.source, &addressor.database)
                                .await?;
                        let command = (
                            unique_name.clone(),
                            node.clone(),
                            deployment.to_owned(),
                            exercise.name.to_string(),
                            links,
                            template_id,
                        )
                            .try_to_deployment_command()?;

                        match addressor
                            .distributor
                            .send(Deploy(deployer_type, command, deployers.to_owned()))
                            .await?
                        {
                            anyhow::Result::Ok(client_response) => {
                                let id = Identifier::try_from(client_response)?.value;

                                deployment_element.status = ElementStatus::Success;
                                deployment_element.handler_reference = Some(id);
                                addressor
                                    .database
                                    .send(UpdateDeploymentElement(
                                        exercise.id,
                                        deployment_element.clone(),
                                        true,
                                    ))
                                    .await??;

                                Ok((node.clone(), deployment_element, template_id))
                            }
                            Err(error) => {
                                deployment_element.status = ElementStatus::Failed;
                                addressor
                                    .database
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

            deployment_results.push(tranche_results);
        }
        let vm_nodes: Vec<(Node, DeploymentElement, Uuid)> = deployment_results
            .concat()
            .into_iter()
            .filter_map(|(node, deployment_element, potential_template_id)| {
                if node.type_field == NodeType::VM {
                    potential_template_id.map(|template_id| (node, deployment_element, template_id))
                } else {
                    None
                }
            })
            .collect();

        Ok(vm_nodes)
    }
}

#[async_trait]
pub trait RemoveableNodes {
    async fn undeploy_nodes(
        self,
        addressor: &Addressor,
        deployers: &[String],
        exercise_id: &Uuid,
    ) -> Result<()>;
}

#[async_trait]
impl RemoveableNodes for Vec<DeploymentElement> {
    async fn undeploy_nodes(
        self,
        addressor: &Addressor,
        deployers: &[String],
        exercise_id: &Uuid,
    ) -> Result<()> {
        try_join_all(self.iter().map(|element| async move {
            match element.deployer_type {
                DeployerType(GrpcDeployerType::VirtualMachine | GrpcDeployerType::Switch) => {
                    if let Some(handler_reference) = &element.handler_reference {
                        let mut element_update = element.clone();

                        return match addressor
                            .distributor
                            .send(UnDeploy(
                                element.deployer_type.0,
                                handler_reference.to_string(),
                                deployers.to_owned(),
                            ))
                            .await?
                        {
                            anyhow::Result::Ok(_) => {
                                element_update.status = ElementStatus::Removed;
                                addressor
                                    .database
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
                                addressor
                                    .database
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
