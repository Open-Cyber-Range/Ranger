use actix::Actor;
use anyhow::Result;
use lazy_static::lazy_static;
use ranger_grpc::Identifier;
use sdl_parser::Scenario;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::{
    configuration::read_configuration,
    node::{CreateNode, NodeClient},
};

lazy_static! {
    static ref DEPLOYED_NODES: Mutex<Vec<Nodest>> = Mutex::new(vec![]);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nodest {
    pub type_field: String,
    pub id: String,
}

pub async fn deploy_scenario(scenario: Scenario) -> Result<()> {
    if let Some(vmlist) = scenario.infrastructure {
        for (name, vm) in vmlist {
            if let Some(source) = vm.source {
                if let Some(template) = source.template {
                    let grpcnode = ranger_grpc::Node {
                        name,
                        exercise_name: scenario.name.clone(),
                        template_name: template,
                    };
                    println!("Deploying node: {:?}", grpcnode.name);
                    let identifier = deploy_vm(grpcnode).await?;
                    let node = Nodest {
                        type_field: "VM".to_string(),
                        id: identifier.value,
                    };
                    DEPLOYED_NODES.lock().unwrap().push(node);
                }
            }
        }
    }
    println!("{:?}", DEPLOYED_NODES.lock());
    Ok(())
}

pub async fn deploy_vm(node: ranger_grpc::Node) -> Result<Identifier> {
    let configuration = read_configuration(std::env::args().collect())?;
    if let Some(deployer_address) = configuration.node_deployer_addresses.into_iter().next() {
        let node_deployer_client = NodeClient::new(deployer_address.clone()).await?.start();
        let identifier_result = node_deployer_client
            .send(CreateNode(node.clone()))
            .await??;
        return Ok(identifier_result);
    }
    return Err(anyhow::anyhow!("List is empty"));
}
