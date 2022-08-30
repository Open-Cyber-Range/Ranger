use crate::{
    node::GetCapabilities,
    node::NodeClient,
    services::deployment::{
        initiate_node_clients, AddDeploymentGroups, DeploymentGroup, DeploymentGroupMap,
        DeploymentManager, NodeClientFilter,
    },
    templater::{initiate_template_clients, TemplateClientFilter},
};

use actix::{Actor, Addr, Context, Handler, Message, MessageResponse};
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

#[derive(Debug, Default, Serialize, Clone)]
pub struct DeployerGroup {
    pub machiners: HashMap<String, String>,
    pub switchers: HashMap<String, String>,
    pub templaters: HashMap<String, String>,
}

impl DeployerGroup {
    pub async fn start(&self) -> DeploymentGroup {
        let machiners = join_all(initiate_node_clients(self.machiners.clone())).await;
        let switchers = join_all(initiate_node_clients(self.switchers.clone())).await;
        let templaters = join_all(initiate_template_clients(self.templaters.clone())).await;
        DeploymentGroup {
            machiners: machiners.filter_node_clients(),
            switchers: switchers.filter_node_clients(),
            templaters: templaters.filter_template_clients(),
        }
    }

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

#[derive(Debug, Default, Clone, Serialize, MessageResponse)]
pub struct DeployerGroups(pub HashMap<String, DeployerGroup>);

impl DeployerGroups {
    pub fn new() -> Self {
        DeployerGroups(HashMap::new())
    }

    pub async fn start_all(
        deployer_grouper_address: Addr<DeployerGroups>,
        deployment_manager_address: Addr<DeploymentManager>,
    ) -> Result<()> {
        let deployer_groups = deployer_grouper_address.send(GetDeployerGroups).await?;
        let deployment_actors = join_all(
            deployer_groups
                .0
                .into_iter()
                .map(|(name, deployer_group)| async move { (name, deployer_group.start().await) }),
        )
        .await;

        let mut deployment_groups: DeploymentGroupMap = HashMap::new();
        deployment_actors
            .iter()
            .for_each(|(name, deployment_group)| {
                deployment_groups.insert(name.to_string(), deployment_group.clone());
            });
        deployment_manager_address
            .send(AddDeploymentGroups(deployment_groups))
            .await?;

        Ok(())
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
