use crate::node::{CreateNode, NodeClient};
use crate::services::scheduler::{CreateDeploymentSchedule, Scheduler};
use crate::templater::{Template, TemplateClient};
use crate::utilities::deployment::Deployment;
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures::{future::try_join_all, Future};
use log::{error, info};
use ranger_grpc::{NodeDeployment, NodeIdentifier};
use sdl_parser::{node::NodeType, Scenario};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Message, Debug)]
#[rtype(result = "Result<Uuid>")]
pub struct CreateDeployment(pub(crate) String, pub(crate) Scenario);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DeleteDeployment(pub(crate) String);

pub type DeploymentGroupMap = HashMap<String, DeploymentGroup>;

#[derive(Debug, Clone)]
pub struct DeploymentManager {
    pub scheduler: Addr<Scheduler>,
    pub exercises: HashMap<String, HashMap<Uuid, Vec<(NodeIdentifier, String)>>>,
    pub deployment_groups: DeploymentGroupMap,
}

#[derive(Debug, Default, Clone)]
pub struct DeploymentGroup {
    pub machiners: HashMap<String, Addr<NodeClient>>,
    pub switchers: HashMap<String, Addr<NodeClient>>,
    pub templaters: HashMap<String, Addr<TemplateClient>>,
}

impl DeploymentManager {
    pub fn new(scheduler: Addr<Scheduler>) -> Self {
        Self {
            scheduler,
            exercises: HashMap::new(),
            deployment_groups: HashMap::new(),
        }
    }

    fn get_deployment_group(&self, name: &str) -> Result<DeploymentGroup> {
        self.deployment_groups
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow!("Deployment group not found"))
    }

    async fn deploy(
        deployment_group: DeploymentGroup,
        scenario: &Scenario,
        scheduler_address: Addr<Scheduler>,
    ) -> Result<Uuid> {
        let deployment_id = Uuid::new_v4();
        let exercise_name = format!("{}-{}", scenario.name, deployment_id);
        let template_id_map = scenario
            .template_nodes(&deployment_group.templaters)
            .await?;

        let deployment_schedule = scheduler_address
            .send(CreateDeploymentSchedule(scenario.clone()))
            .await??;
        let template_id_map = &template_id_map;
        let exercise_name = &exercise_name;
        let deployment_group = &deployment_group;

        try_join_all(
            deployment_schedule
                .into_iter()
                .map(move |tranche| async move {
                    let template_id_map = template_id_map.clone();
                    let exercise_name = exercise_name.clone();

                    let template_id_map = &template_id_map;
                    let exercise_name = &exercise_name;
                    let deployment_group = &deployment_group;
                    try_join_all(tranche.into_iter().map(
                        move |(node_name, display_name, node)| async move {
                            match node.type_field {
                                NodeType::VM => {
                                    deployment_group
                                        .deploy_vms(vec![node.to_deployment(
                                            &node_name,
                                            &display_name,
                                            template_id_map,
                                            exercise_name,
                                        )?])
                                        .await?;
                                }
                                NodeType::Switch => {
                                    deployment_group
                                        .deploy_switches(vec![node.to_deployment(
                                            &node_name,
                                            &display_name,
                                            template_id_map,
                                            exercise_name,
                                        )?])
                                        .await?;
                                }
                            }
                            info!("Node: {node_name} successful");
                            Ok::<()>(())
                        },
                    ))
                    .await?;

                    Ok::<()>(())
                }),
        )
        .await?;
        info!("Deployment {deployment_id} successful");
        Ok(deployment_id)
    }
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct AddDeploymentGroups(pub(crate) DeploymentGroupMap);

impl Actor for DeploymentManager {
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
        let scenario = msg.1;
        let requested_deployer_group_name = msg.0;
        info!("Using deployment group: {}", &requested_deployer_group_name);
        let deployment_group = self
            .get_deployment_group(&requested_deployer_group_name)
            .unwrap();
        let scheduler_address = self.scheduler.clone();

        Box::pin(
            async move {
                DeploymentManager::deploy(deployment_group, &scenario, scheduler_address).await
            }
            .into_actor(self)
            .map(move |result, _act, _| {
                if let Result::Ok(deployment_id) = result {
                    Ok(deployment_id)
                } else {
                    error!("Deployment failed: {:?}", result.as_ref().err());
                    Err(anyhow!("Deployment failed {:?}", result.err()))
                }
            }),
        )
    }
}

impl Handler<AddDeploymentGroups> for DeploymentManager {
    type Result = ();
    fn handle(&mut self, msg: AddDeploymentGroups, _: &mut Context<Self>) -> Self::Result {
        self.deployment_groups.extend(msg.0);
    }
}
