use anyhow::{anyhow, Result};
use log::info;
use ranger_grpc::{Configuration, NodeDeployment};
use ranger_grpc::{
    DeploymentParameters, Node as GrpcNode, NodeIdentifier, NodeType as GrpcNodeType,
};
use sdl_parser::infrastructure::InfraNode;
use sdl_parser::node::{Node, NodeType};
use std::collections::HashMap;

pub trait Deployment
where
    Self: Sized,
{
    fn new_switch(&self, name: &str, exercise_name: &str, links: Vec<String>) -> NodeDeployment;

    fn new_virtual_machine(
        &self,
        name: &str,
        exercise_name: &str,
        template_id: &str,
        links: Vec<String>,
    ) -> NodeDeployment;

    fn get_type(&self) -> NodeType;

    fn to_deployment(
        &self,
        node_name: &str,
        display_name: &str,
        template_ids_map: &HashMap<String, String>,
        infranode_map: &HashMap<String, InfraNode>,
        exercise_name: &str,
    ) -> Result<NodeDeployment> {
        match self.get_type() {
            sdl_parser::node::NodeType::VM => {
                let template_id = template_ids_map
                    .get(node_name)
                    .ok_or_else(|| anyhow!("No template found for node: {}", node_name))?;
                let links = infranode_map
                    .get(node_name)
                    .ok_or_else(|| anyhow!("Infranode not found: {}", node_name))?
                    .links.clone()
                    .ok_or_else(|| anyhow!("No links found for node: {}", node_name))?;

                info!("links for node {}: {:?}", node_name, links);
                Ok(self.new_virtual_machine(display_name, exercise_name, template_id, links))
            }
            sdl_parser::node::NodeType::Switch => {
                let links = infranode_map
                    .get(node_name)
                    .ok_or_else(|| anyhow!("Infranode not found: {}", node_name))?
                    .links.clone()
                    .ok_or_else(|| anyhow!("No links found for node: {}", node_name))?;
                Ok(self.new_switch(display_name, exercise_name, links))
            }
        }
    }
}

impl Deployment for Node {
    fn get_type(&self) -> NodeType {
        self.type_field.clone()
    }

    fn new_switch(&self, name: &str, exercise_name: &str, links: Vec<String>) -> NodeDeployment {
        NodeDeployment {
            parameters: Some(DeploymentParameters {
                name: name.to_string(),
                exercise_name: exercise_name.to_string(),
                template_id: "".to_string(),
                links,
            }),
            node: Some(GrpcNode {
                identifier: Some(NodeIdentifier {
                    identifier: None,
                    node_type: GrpcNodeType::Switch.into(),
                }),
                configuration: None,
            }),
        }
    }

    fn new_virtual_machine(
        &self,
        name: &str,
        exercise_name: &str,
        template_id: &str,
        links: Vec<String>,
    ) -> NodeDeployment {
        NodeDeployment {
            parameters: Some(DeploymentParameters {
                name: name.to_string(),
                exercise_name: exercise_name.to_string(),
                template_id: template_id.to_string(),
                links,
            }),
            node: Some(GrpcNode {
                identifier: Some(NodeIdentifier {
                    identifier: None,
                    node_type: GrpcNodeType::Vm.into(),
                }),
                configuration: self.resources.as_ref().map(|resources| Configuration {
                    cpu: resources.cpu,
                    ram: resources.ram,
                }),
            }),
        }
    }
}
