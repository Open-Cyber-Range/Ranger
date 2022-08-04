use crate::node::{CreateNode, NodeClient};
use crate::templater::TemplateClient;
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use futures::{future::try_join_all, Future};
use log::{error, info};
use ranger_grpc::{
    Configuration, DeploymentParameters, Node, NodeDeployment, NodeIdentifier, NodeType,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Message, Debug)]
#[rtype(result = "Result<Uuid>")]
pub struct CreateDeployment(
    pub(crate) (Vec<NodeDeployment>, Vec<NodeDeployment>),
    pub(crate) DeploymentGroup,
    pub String,
    pub Uuid,
);

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
    pub templaters: Vec<Addr<TemplateClient>>,
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

pub trait NodeDeploymentTrait {
    fn initialize_vm(
        &self,
        node: sdl_parser::node::Node,
        node_name: String,
        template_id: String,
        exercise_name: String,
    ) -> Result<NodeDeployment>;

    fn initialize_switch(
        &self,
        node_name: String,
        template_id: String,
        exercise_name: String,
    ) -> Result<NodeDeployment>;
}
impl NodeDeploymentTrait for NodeDeployment {
    fn initialize_vm(
        &self,
        node: sdl_parser::node::Node,
        node_name: String,
        template_id: String,
        exercise_name: String,
    ) -> Result<NodeDeployment> {
        let resources = node
            .resources
            .ok_or_else(|| anyhow!("Resources field is missing"))?;
        let deployment = NodeDeployment {
            parameters: Some(DeploymentParameters {
                name: node_name,
                exercise_name,
                template_id,
            }),
            node: Some(Node {
                identifier: Some(NodeIdentifier {
                    identifier: None,
                    node_type: NodeType::Vm.into(),
                }),
                configuration: Some(Configuration {
                    cpu: resources.cpu,
                    ram: resources.ram,
                }),
            }),
        };
        Ok(deployment)
    }

    fn initialize_switch(
        &self,
        node_name: String,
        template_id: String,
        exercise_name: String,
    ) -> Result<NodeDeployment> {
        Ok(NodeDeployment {
            parameters: Some(DeploymentParameters {
                name: node_name,
                exercise_name,
                template_id,
            }),
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

pub async fn deploy_vm(
    node_deployment: NodeDeployment,
    machiner_client: Addr<NodeClient>,
) -> Result<NodeIdentifier> {
    info!(
        "Deploying VM: {}",
        node_deployment
            .parameters
            .as_ref()
            .ok_or_else(|| anyhow!("Error getting parameters"))?
            .name
    );
    machiner_client.send(CreateNode(node_deployment)).await?
}
pub async fn deploy_switch(
    node_deployment: NodeDeployment,
    switcher_client: Addr<NodeClient>,
) -> Result<NodeIdentifier> {
    info!(
        "Deploying Switch: {}",
        node_deployment
            .parameters
            .as_ref()
            .ok_or_else(|| anyhow!("Error getting parameters"))?
            .name
    );
    switcher_client.send(CreateNode(node_deployment)).await?
}

impl DeploymentGroup {
    pub fn deploy_vms<'a>(
        &self,
        node_deployment: Vec<NodeDeployment>,
    ) -> futures::future::TryJoinAll<
        impl Future<Output = Result<(NodeIdentifier, String), anyhow::Error>> + 'a,
    > {
        try_join_all(
            node_deployment
                .into_iter()
                .zip(self.machiners.clone().into_iter().cycle())
                .map(|(node_deployment, machiner_client)| async move {
                    let node_id = deploy_vm(node_deployment.clone(), machiner_client).await?;
                    let node_name = &node_deployment
                        .parameters
                        .as_ref()
                        .ok_or_else(|| anyhow!("Error getting parameters"))?
                        .name;
                    info!("Deployment of VM {} finished", node_name);
                    Ok::<(NodeIdentifier, String)>((node_id, node_name.to_owned()))
                }),
        )
    }

    pub fn deploy_switches<'a>(
        &self,
        node_deployment: Vec<NodeDeployment>,
    ) -> futures::future::TryJoinAll<
        impl Future<Output = Result<(NodeIdentifier, String), anyhow::Error>> + 'a,
    > {
        try_join_all(
            node_deployment
                .into_iter()
                .zip(self.switchers.clone().into_iter().cycle())
                .map(|(node_deployment, switcher_client)| async move {
                    let node_id = deploy_switch(node_deployment.clone(), switcher_client).await?;
                    let node_name = &node_deployment
                        .parameters
                        .as_ref()
                        .ok_or_else(|| anyhow!("Error getting parameters"))?
                        .name;
                    info!("Deployment of VM {} finished", node_name);
                    Ok::<(NodeIdentifier, String)>((node_id, node_name.to_owned()))
                }),
        )
    }
}

impl Handler<CreateDeployment> for DeploymentManager {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: CreateDeployment, _: &mut Context<Self>) -> Self::Result {
        let node_deployments = msg.0;
        let deployment_group = msg.1;
        let exercise_name = msg.2;
        let deployment_id = msg.3;
        Box::pin(
            async move {
                let vm_ids = deployment_group.deploy_vms(node_deployments.0).await?;
                let switch_ids = deployment_group.deploy_switches(node_deployments.1).await?;
                let node_ids: Vec<(NodeIdentifier, String)> =
                    vm_ids.into_iter().chain(switch_ids.into_iter()).collect();
                Ok((deployment_id, node_ids))
            }
            .into_actor(self)
            .map(move |result, act, _| {
                if let Result::Ok((deployment_id, node_ids)) = result {
                    act.nodes
                        .entry(exercise_name)
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
