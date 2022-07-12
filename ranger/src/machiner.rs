use crate::node::{CreateNode, NodeClient};
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use futures::{future::try_join_all, Future};
use log::{error, info};
use ranger_grpc::{DeploymentParameters, Node, NodeDeployment, NodeIdentifier, NodeType};
use sdl_parser::{node, Scenario};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Message, Debug)]
#[rtype(result = "Result<Uuid>")]
pub struct CreateDeployment(pub(crate) Scenario, pub(crate) DeploymentGroup);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DeleteDeployment(pub(crate) String);

#[derive(Debug, Default, Clone)]
pub struct DeploymentManager {
    nodes: HashMap<String, HashMap<Uuid, Vec<(NodeIdentifier, String)>>>,
}

impl DeploymentManager {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Actor for DeploymentManager {
    type Context = Context<Self>;
}
#[derive(Debug, Default, Clone)]
pub struct DeploymentGroup {
    pub machiners: Vec<Addr<NodeClient>>,
    pub switchers: Vec<Addr<NodeClient>>,
    pub templaters: Vec<Addr<NodeClient>>,
}

pub fn initiate_node_clients(
    deployers: HashMap<String, String>,
) -> Vec<impl Future<Output = Result<Addr<NodeClient>, anyhow::Error>>> {
    deployers
        .into_iter()
        .map(|(_, ip)| async { Ok::<Addr<NodeClient>>(NodeClient::new(ip).await?.start()) })
        .collect()
}

pub fn filter_node_clients(
    nodeclient_results: Vec<Result<Addr<NodeClient>, anyhow::Error>>,
) -> Vec<Addr<NodeClient>> {
    nodeclient_results
        .into_iter()
        .filter_map(|node_client| {
            node_client
                .map_err(|error| error!("Error setting up NodeClient: {}", error))
                .ok()
        })
        .collect::<Vec<Addr<NodeClient>>>()
}

trait NodeDeploymentTrait {
    fn set_deployment_parameters(
        &self,
        node: (String, sdl_parser::node::Node),
        exercise_name: String,
    ) -> Result<DeploymentParameters> {
        Ok(DeploymentParameters {
            name: node.0.clone(),
            exercise_name,
            template_name: node
                .1
                .source
                .ok_or_else(|| anyhow!("Source is missing"))?
                .template
                .ok_or_else(|| anyhow!("Template is missing"))?,
        })
    }

    fn initialize_vm(
        &self,
        node: (String, sdl_parser::node::Node),
        exercise_name: String,
    ) -> Result<NodeDeployment> {
        let deployment = NodeDeployment {
            parameters: Some(self.set_deployment_parameters(node, exercise_name)?),
            node: Some(Node {
                identifier: Some(NodeIdentifier {
                    identifier: None,
                    node_type: NodeType::Vm.into(),
                }),
                configuration: None,
            }),
        };
        Ok(deployment)
    }

    fn initialize_switch(
        &self,
        node: (String, sdl_parser::node::Node),
        exercise_name: String,
    ) -> Result<NodeDeployment> {
        Ok(NodeDeployment {
            parameters: Some(self.set_deployment_parameters(node, exercise_name)?),
            node: Some(Node {
                identifier: Some(NodeIdentifier {
                    identifier: None,
                    node_type: NodeType::Switch.into(),
                }),
                configuration: None,
            }),
        })
    }
}
impl NodeDeploymentTrait for NodeDeployment {}

impl DeploymentManager {
    pub fn deploy_vms(
        infrastructure: HashMap<String, node::Node>,
        deployment_group: DeploymentGroup,
        exercise_name: &str,
    ) -> futures::future::TryJoinAll<
        impl Future<Output = Result<(NodeIdentifier, String), anyhow::Error>> + '_,
    > {
        try_join_all(
            infrastructure
                .into_iter()
                .zip(deployment_group.machiners.into_iter().cycle())
                .map(|(node, machiner_client)| async move {
                    match node.1.type_field {
                        node::NodeType::VM => {
                            info!("Deploying VM: {}", node.0);
                            let node_id = machiner_client
                                .send(CreateNode(
                                    NodeDeployment::default()
                                        .initialize_vm(node.clone(), exercise_name.to_string())?,
                                ))
                                .await??;
                            info!("Deployment of VM {} finished", node.0);
                            Ok::<(NodeIdentifier, String)>((node_id, node.0))
                        }
                        _ => Err(anyhow!("Node type not supported for machiner deployment")),
                    }
                }),
        )
    }
    pub fn deploy_switches(
        infrastructure: HashMap<String, node::Node>,
        deployment_group: DeploymentGroup,
        exercise_name: &str,
    ) -> futures::future::TryJoinAll<
        impl Future<Output = Result<(NodeIdentifier, String), anyhow::Error>> + '_,
    > {
        try_join_all(
            infrastructure
                .into_iter()
                .zip(deployment_group.switchers.into_iter().cycle())
                .map(|(node, switcher_client)| async move {
                    match node.1.type_field {
                        node::NodeType::Network => {
                            info!("Deploying Switch: {}", node.0);
                            let node_id =
                                switcher_client
                                    .send(CreateNode(NodeDeployment::default().initialize_switch(
                                        node.clone(),
                                        exercise_name.to_string(),
                                    )?))
                                    .await??;
                            info!("Deployment of Switch {} finished", node.0);
                            Ok::<(NodeIdentifier, String)>((node_id, node.0))
                        }
                        _ => Err(anyhow!("Node type not supported for switcher deployment")),
                    }
                }),
        )
    }
}

impl Handler<CreateDeployment> for DeploymentManager {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: CreateDeployment, _: &mut Context<Self>) -> Self::Result {
        let scenario = msg.0.clone();
        let deployment_group = msg.1;
        Box::pin(
            async move {
                let deployment_id = Uuid::new_v4();
                let exercise_name = format!("{}-{}", scenario.name, deployment_id);
                let exercise_name = exercise_name.as_str();
                let node_ids: Vec<(NodeIdentifier, String)> = if let Some(infrastructure) =
                    scenario.infrastructure
                {
                    DeploymentManager::deploy_vms(infrastructure, deployment_group, exercise_name)
                        .await?
                } else {
                    Vec::new()
                };
                Ok((deployment_id, node_ids))
            }
            .into_actor(self)
            .map(move |result, act, _| {
                if let Result::Ok((deployment_id, node_ids)) = result {
                    act.nodes
                        .entry(msg.0.name)
                        .or_insert_with(HashMap::new)
                        .insert(deployment_id, node_ids.clone());
                    info!(
                        "Deployment {deployment_id} successful, {:?} nodes deployed",
                        node_ids.len()
                    );
                    Ok(deployment_id)
                } else {
                    error!("Deployment failed: {:?}", result.as_ref().err());
                    Err(anyhow!("Deployment failed {:?}", result.err()))
                }
            }),
        )
    }
}
