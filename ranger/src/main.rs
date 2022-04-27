mod configuration;
mod node;

use actix::Actor;
use anyhow::Error;

use crate::{
    configuration::read_configuration,
    node::{CreateNode, DeleteNode, NodeClient},
};

#[actix::main]
async fn main() -> Result<(), Error> {
    let configuration = read_configuration(std::env::args().collect())?;
    for deployer_address in configuration.node_deployer_addresses {
        let node_deployer_client = NodeClient::new(deployer_address.clone()).await?.start();
        println!("Deploying node at: {}", deployer_address);
        let node_id = node_deployer_client
            .send(CreateNode(ranger_grpc::Node {
                name: "some-name".to_string(),
                exercise_name: "some-exercise-name".to_string(),
                template_name: "debian10".to_string(),
            }))
            .await??;
        println!("Node deployed, now deleting");
        node_deployer_client.send(DeleteNode(node_id)).await??;
        println!("Node deleted");
    }
    Ok(())
}
