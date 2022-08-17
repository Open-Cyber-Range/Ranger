use crate::node::{CreateNode, NodeClient};
use crate::templater::TemplateClient;
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, MessageResponse, ResponseActFuture,
    WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
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
    pub(crate) Vec<(Vec<NodeDeployment>, Vec<NodeDeployment>)>,
    pub(crate) DeploymentGroup,
    pub String,
    pub Uuid,
);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DeleteDeployment(pub(crate) String);

#[derive(Debug, Default, Clone)]
pub struct DeploymentManager {
    pub exercises: HashMap<String, HashMap<Uuid, Vec<(NodeIdentifier, String)>>>,
    pub deployment_groups: DeploymentGroups,
}

#[derive(Debug, Default, Clone)]
pub struct DeploymentGroup {
    pub machiners: HashMap<String, Addr<NodeClient>>,
    pub switchers: HashMap<String, Addr<NodeClient>>,
    pub templaters: HashMap<String, Addr<TemplateClient>>,
}

#[derive(Debug, Default, Clone, MessageResponse)]
pub struct DeploymentGroups(pub HashMap<String, DeploymentGroup>);

impl DeploymentManager {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct AddDeploymentGroups(pub(crate) DeploymentGroups);

#[derive(Debug, Message, PartialEq)]
#[rtype(result = "DeploymentGroups")]
pub struct GetDeploymentGroups;

#[derive(Message, Debug, PartialEq)]
#[rtype(result = "Result<(String, DeploymentGroup)>")]
pub struct FindDeploymentGroupByName(pub(crate) String);

impl DeploymentGroups {
    pub fn new() -> Self {
        DeploymentGroups(HashMap::new())
    }
}

impl Actor for DeploymentManager {
    type Context = Context<Self>;
}

impl Actor for DeploymentGroups {
    type Context = Context<Self>;
}

pub fn initiate_node_clients(
    deployers: HashMap<String, String>,
) -> Vec<impl Future<Output = Result<(String, Addr<NodeClient>), anyhow::Error>>> {
    deployers
        .into_iter()
        .map(|(name, ip)| async {
            Ok::<(String, Addr<NodeClient>)>((name, NodeClient::new(ip).await?.start()))
        })
        .collect()
}

pub type NodeClientResults = Vec<Result<(String, Addr<NodeClient>), anyhow::Error>>;
pub trait NodeClientFilter {
    fn filter_node_clients(self) -> HashMap<String, Addr<NodeClient>>;
}
impl NodeClientFilter for NodeClientResults {
    fn filter_node_clients(self) -> HashMap<String, Addr<NodeClient>> {
        self.into_iter()
            .filter_map(|node_client| {
                node_client
                    .map_err(|error| error!("Error setting up NodeClient: {}", error))
                    .ok()
            })
            .collect::<HashMap<String, Addr<NodeClient>>>()
    }
}

pub trait NodeDeploymentTrait {
    fn initialize_vm(
        &self,
        node: sdl_parser::node::Node,
        node_name: String,
        template_id: String,
        exercise_name: &str,
    ) -> Result<NodeDeployment>;

    fn initialize_switch(&self, node_name: String, exercise_name: &str) -> Result<NodeDeployment>;

    fn create_from_nodes(
        &self,
        nodes: HashMap<String, sdl_parser::node::Node>,
        template_id: HashMap<String, String>,
        exercise_name: &str,
    ) -> Result<Vec<NodeDeployment>>;
}
impl NodeDeploymentTrait for NodeDeployment {
    fn initialize_vm(
        &self,
        node: sdl_parser::node::Node,
        node_name: String,
        template_id: String,
        exercise_name: &str,
    ) -> Result<NodeDeployment> {
        let resources = node
            .resources
            .ok_or_else(|| anyhow!("Resources field is missing"))?;
        let deployment = NodeDeployment {
            parameters: Some(DeploymentParameters {
                name: node_name,
                exercise_name: exercise_name.to_string(),
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

    fn initialize_switch(&self, node_name: String, exercise_name: &str) -> Result<NodeDeployment> {
        Ok(NodeDeployment {
            parameters: Some(DeploymentParameters {
                name: node_name,
                exercise_name: exercise_name.to_string(),
                template_id: "".to_string(),
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
    fn create_from_nodes(
        &self,
        nodes: HashMap<String, sdl_parser::node::Node>,
        template_id: HashMap<String, String>,
        exercise_name: &str,
    ) -> Result<Vec<NodeDeployment>> {
        nodes
            .into_iter()
            .map(|(node_name, node)| {
                let template_id = template_id
                    .get(&node_name)
                    .ok_or_else(|| anyhow!("Template ID for node {} is missing", node_name))?;
                match node.type_field {
                    sdl_parser::node::NodeType::VM => {
                        self.initialize_vm(node, node_name, template_id.to_string(), exercise_name)
                    }
                    sdl_parser::node::NodeType::Switch => {
                        self.initialize_switch(node_name, exercise_name)
                    }
                }
            })
            .collect::<Result<Vec<NodeDeployment>>>()
    }
}

#[async_trait]
pub trait Sender {
    async fn deploy_node(&self, node_client: Addr<NodeClient>) -> Result<NodeIdentifier>;
}
#[async_trait]
impl Sender for NodeDeployment {
    async fn deploy_node(&self, node_client: Addr<NodeClient>) -> Result<NodeIdentifier> {
        info!(
            "Deploying VM: {}",
            &self
                .parameters
                .as_ref()
                .ok_or_else(|| anyhow!("Error getting parameters"))?
                .name
        );
        node_client.send(CreateNode(self.to_owned())).await?
    }
}

impl DeploymentGroup {
    pub fn deploy_vms(
        &self,
        node_deployment: Vec<NodeDeployment>,
    ) -> futures::future::TryJoinAll<
        impl Future<Output = Result<(NodeIdentifier, String), anyhow::Error>> + '_,
    > {
        try_join_all(
            node_deployment
                .into_iter()
                .zip(self.machiners.values().into_iter().cycle())
                .map(|(node_deployment, machiner_client)| async move {
                    let node_id = node_deployment.deploy_node(machiner_client.clone()).await?;
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

    pub fn deploy_switches(
        &self,
        node_deployment: Vec<NodeDeployment>,
    ) -> futures::future::TryJoinAll<
        impl Future<Output = Result<(NodeIdentifier, String), anyhow::Error>> + '_,
    > {
        try_join_all(
            node_deployment
                .into_iter()
                .zip(self.switchers.values().into_iter().cycle())
                .map(|(node_deployment, switcher_client)| async move {
                    let node_id = node_deployment.deploy_node(switcher_client.clone()).await?;
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
        let linked_node_deployments = msg.0;
        let deployment_group = msg.1;
        let exercise_name = msg.2;
        let deployment_id = msg.3;
        Box::pin(
            async move {
                let node_ids = try_join_all(linked_node_deployments.into_iter().map(
                    |node_deployment_group| async {
                        let vm_ids = deployment_group.deploy_vms(node_deployment_group.0).await?;
                        let switch_ids = deployment_group
                            .deploy_switches(node_deployment_group.1)
                            .await?;
                        let node_ids = vm_ids.into_iter().chain(switch_ids.into_iter()).collect();
                        Ok::<Vec<(NodeIdentifier, String)>>(node_ids)
                    },
                ))
                .await?;
                Ok((deployment_id, node_ids))
            }
            .into_actor(self)
            .map(move |result, act, _| {
                if let Result::Ok((deployment_id, node_id_groups)) = result {
                    for node_ids in node_id_groups {
                        act.exercises
                            .entry(exercise_name.clone())
                            .or_insert_with(HashMap::new)
                            .insert(deployment_id, node_ids.clone());
                    }
                    let node_count = act
                        .exercises
                        .get(&exercise_name)
                        .ok_or_else(|| anyhow!("Exercise not found"))?
                        .get(&deployment_id)
                        .ok_or_else(|| anyhow!("Deployment id not found"))?
                        .len();
                    info!("Deployment {deployment_id} successful, nodes deployed: {node_count}");
                    Ok(deployment_id)
                } else {
                    error!("Deployment failed: {:?}", result.as_ref().err());
                    Err(anyhow!("Deployment failed {:?}", result.err()))
                }
            }),
        )
    }
}

impl Handler<FindDeploymentGroupByName> for DeploymentManager {
    type Result = Result<(String, DeploymentGroup)>;
    fn handle(&mut self, msg: FindDeploymentGroupByName, _: &mut Context<Self>) -> Self::Result {
        let deployment_groups = self.to_owned().deployment_groups;
        let deployer = deployment_groups
            .0
            .into_iter()
            .find(|deployer_group| deployer_group.0.eq_ignore_ascii_case(&msg.0))
            .ok_or_else(|| anyhow!("DeploymentGroup not found"))?;
        Ok(deployer)
    }
}

impl Handler<AddDeploymentGroups> for DeploymentManager {
    type Result = ();
    fn handle(&mut self, msg: AddDeploymentGroups, _: &mut Context<Self>) -> Self::Result {
        self.deployment_groups.0.extend(msg.0 .0);
    }
}

impl Handler<GetDeploymentGroups> for DeploymentManager {
    type Result = DeploymentGroups;
    fn handle(&mut self, _: GetDeploymentGroups, _: &mut Context<Self>) -> Self::Result {
        self.deployment_groups.to_owned()
    }
}
