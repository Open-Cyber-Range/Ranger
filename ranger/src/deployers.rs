use crate::{capability::GetCapabilities, node::NodeClient};

use actix::{Actor, Context, Handler, Message, MessageResponse};
use anyhow::{anyhow, Result};
use futures::future::join_all;
use log::error;
use ranger_grpc::{capabilities::DeployerTypes, Capabilities};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct AddDeployerGroups(pub(crate) DeployerGroups);

#[derive(Message, Debug, PartialEq)]
#[rtype(result = "DeployerGroups")]
pub struct GetDeployerGroups;

#[derive(Debug, Default, Clone)]
pub struct Deployer {
    pub deployer_name: String,
    pub deployer_ip: String,
    pub capabilities: Option<Capabilities>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Clone)]
pub struct DeployerGroup {
    pub machiners: HashMap<String, String>,
    pub switchers: HashMap<String, String>,
    pub templaters: HashMap<String, String>,
}

impl DeployerGroup {
    pub fn insert_by_capability(&mut self, deployer: &Deployer) {
        if let Some(capabilities) = &deployer.capabilities {
            capabilities
                .values()
                .into_iter()
                .for_each(|capability| match capability {
                    DeployerTypes::VirtualMachine => {
                        self.machiners.insert(
                            deployer.deployer_name.to_owned(),
                            deployer.deployer_ip.to_owned(),
                        );
                    }
                    DeployerTypes::Switch => {
                        self.switchers.insert(
                            deployer.deployer_name.to_owned(),
                            deployer.deployer_ip.to_owned(),
                        );
                    }
                    DeployerTypes::Template => {
                        self.templaters.insert(
                            deployer.deployer_name.to_owned(),
                            deployer.deployer_ip.to_owned(),
                        );
                    }
                });
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, MessageResponse)]
pub struct DeployerGroups(pub HashMap<String, DeployerGroup>);

impl DeployerGroups {
    pub fn new() -> Self {
        DeployerGroups(HashMap::new())
    }
    pub fn initialize_with_group_names(
        deployment_groups: &HashMap<String, Vec<String>>,
    ) -> DeployerGroups {
        let mut deployer_groups = DeployerGroups::default();
        deployment_groups.iter().for_each(|deployer_group| {
            deployer_groups
                .0
                .insert(deployer_group.0.to_owned(), DeployerGroup::default());
        });
        deployer_groups
    }
}

impl Actor for DeployerGroups {
    type Context = Context<Self>;
}

impl Handler<AddDeployerGroups> for DeployerGroups {
    type Result = ();
    fn handle(&mut self, msg: AddDeployerGroups, _: &mut Context<Self>) -> Self::Result {
        self.0.extend(msg.0 .0);
    }
}

impl Handler<GetDeployerGroups> for DeployerGroups {
    type Result = DeployerGroups;
    fn handle(&mut self, _: GetDeployerGroups, _: &mut Context<Self>) -> Self::Result {
        self.to_owned()
    }
}

fn filter_capable_deployers(capability_results: Vec<Result<Deployer>>) -> Vec<Deployer> {
    capability_results
        .into_iter()
        .filter_map(|result| {
            result
                .map_err(|error| error!("Error getting deployer capability: {}", error))
                .ok()
        })
        .collect()
}

pub async fn get_deployer_capabilities(
    deployers: HashMap<String, String>,
) -> Result<Vec<Deployer>> {
    let capability_results = join_all(deployers.into_iter().map(|deployer| async move {
        let deployer_client = NodeClient::new(deployer.1.to_owned()).await?.start();
        let capabilities = deployer_client.send(GetCapabilities).await??;
        Ok(Deployer {
            deployer_name: deployer.0.to_owned(),
            deployer_ip: deployer.1.to_owned(),
            capabilities: Some(capabilities),
        })
    }))
    .await;
    let verified_deployers = filter_capable_deployers(capability_results);
    if verified_deployers.is_empty() {
        error!("No deployers found with capabilities");
        return Err(anyhow!("No deployers found with capabilities"));
    }
    Ok(verified_deployers)
}
