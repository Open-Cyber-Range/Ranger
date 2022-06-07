use crate::node::{CreateNode, NodeClient};
use actix::{
    Actor, ActorFutureExt, Addr, Context, Handler, Message, ResponseActFuture, WrapFuture,
};
use anyhow::{anyhow, Ok, Result};
use futures::future::try_join_all;
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
    pub fn new(node_client_address: Addr<NodeClient>) -> Self {
        Self {
            nodes: HashMap::new(),
            node_client_address,
        }
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
                                        template_name: node.source.unwrap().template.unwrap(),
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
                println!("result: {:?}", result);
                if let core::result::Result::Ok((deployment_id, node_ids)) = result {
                    act.nodes
                        .entry(scenario.name)
                        .or_insert_with(HashMap::new)
                        .insert(deployment_id, node_ids);
                    Ok(deployment_id)
                } else {
                    Err(anyhow!("Deployment failed"))
                }
            }),
        )
    }
}

#[cfg(test)]
mod tests {

}
