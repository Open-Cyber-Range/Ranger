use crate::deployers::{DeployerGroups, GetDeployerGroups};
use crate::node::{CreateNode, NodeClient};
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use futures::future::try_join_all;
use log::info;
use ranger_grpc::{DeploymentParameters, Node, NodeDeployment, NodeIdentifier, NodeType};
use sdl_parser::Scenario;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Message, Debug)]
#[rtype(result = "Result<Uuid>")]
pub struct CreateDeployment(pub(crate) Scenario);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DeleteDeployment(pub(crate) String);

#[derive(Debug, Clone)]
pub struct DeploymentManager {
    nodes: HashMap<String, HashMap<Uuid, Vec<(NodeIdentifier, String)>>>,
    node_client_address: Addr<NodeClient>,
}

impl DeploymentManager {
    pub async fn new(
        deployer_actor_address: Addr<DeployerGroups>,
        deployment_group_name: String,
    ) -> Result<Self> {
        let validated_deployer_groups = deployer_actor_address.send(GetDeployerGroups).await?;
        if let Some(deployer_group) = validated_deployer_groups.0.get(&deployment_group_name) {
            if let Some(machiner) = deployer_group.machiners.values().next() {
                return Ok(DeploymentManager {
                    nodes: HashMap::new(),
                    node_client_address: NodeClient::new(machiner.to_string()).await?.start(),
                });
            }
            return Err(anyhow!("No machiners found"));
        }
        Err(anyhow!(
            "Deployer group named {deployment_group_name} not found"
        ))
    }
}

impl Actor for DeploymentManager {
    type Context = Context<Self>;
}

impl Handler<CreateDeployment> for DeploymentManager {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: CreateDeployment, _: &mut Context<Self>) -> Self::Result {
        let scenario = msg.0;
        let client_address = self.node_client_address.clone();
        let scenario_name = scenario.name.clone();
        Box::pin(
            async move {
                let deployment_id = Uuid::new_v4();
                let exercise_name = format!("{}-{}", scenario_name, deployment_id);
                let node_ids: Vec<(NodeIdentifier, String)> =
                    if let Some(infrastructure) = scenario.infrastructure {
                        try_join_all(infrastructure.into_iter().map(|(node_name, node)| async {
                            let node_id = client_address
                                .send(CreateNode(NodeDeployment {
                                    parameters: Some(DeploymentParameters {
                                        name: node_name.clone(),
                                        exercise_name: exercise_name.clone(),
                                        template_name: node
                                            .source
                                            .ok_or_else(|| anyhow!("Source is missing"))?
                                            .template
                                            .ok_or_else(|| anyhow!("Template is missing"))?,
                                    }),
                                    node: Some(Node {
                                        identifier: Some(NodeIdentifier {
                                            identifier: None,
                                            node_type: NodeType::Vm.into(),
                                        }),
                                        configuration: None,
                                    }),
                                }))
                                .await??;
                            Ok::<(NodeIdentifier, String)>((node_id, node_name))
                        }))
                        .await?
                    } else {
                        Vec::new()
                    };
                Ok((deployment_id, node_ids))
            }
            .into_actor(self)
            .map(move |result, act, _| {
                if let core::result::Result::Ok((deployment_id, node_ids)) = result {
                    info!(
                        "Successful deployment id: {deployment_id}, deployed {} nodes",
                        &node_ids.len()
                    );
                    act.nodes
                        .entry(scenario.name)
                        .or_insert_with(HashMap::new)
                        .insert(deployment_id, node_ids);
                    Ok(deployment_id)
                } else {
                    Err(anyhow!(
                        "Deployment failed: {:?}",
                        result
                            .err()
                            .unwrap_or_else(|| anyhow!("DeploymentManager error"))
                    ))
                }
            }),
        )
    }
}
