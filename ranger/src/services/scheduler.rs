use actix::{Actor, Handler, Message};
use anyhow::{anyhow, Ok, Result};
use sdl_parser::{
    node::{Node, NodeType},
    Scenario,
};

#[derive(Default)]
pub struct Scheduler;

impl Actor for Scheduler {
    type Context = actix::Context<Self>;
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler
    }
}

#[derive(Message, Debug, PartialEq)]
#[rtype(result = "Result<Vec<Vec<(String, String, Node)>>>")]
pub struct CreateDeploymentSchedule(pub(crate) Scenario);

impl CreateDeploymentSchedule {
    fn create_node_name(node: &Node, node_name: String, count: u32) -> String {
        match node.type_field {
            NodeType::VM => format!("{node_name}-{count}"),
            _ => node_name,
        }
    }

    pub fn generate(&self) -> Result<Vec<Vec<(String, String, Node)>>> {
        let scenario = &self.0;
        let dependencies = scenario.get_dependencies()?;
        let tranches = dependencies.generate_tranches()?;

        if let Some(infrastructure) = &scenario.infrastructure {
            if let Some(nodes) = &scenario.nodes {
                let mut node_deployments: Vec<Vec<(String, String, Node)>> = Vec::new();
                tranches.iter().try_for_each(|tranche| {
                    let mut new_tranche = Vec::new();
                    tranche.iter().try_for_each(|node_name| {
                        if let Some(infra_value) = infrastructure.get(node_name) {
                            let node_value =
                                nodes.get(node_name).ok_or_else(|| anyhow!("Node value"))?;
                            for n in 0..infra_value.count {
                                new_tranche.push((
                                    node_name.to_string(),
                                    Self::create_node_name(node_value, node_name.clone(), n),
                                    node_value.clone(),
                                ));
                            }
                        }
                        Ok(())
                    })?;
                    node_deployments.push(new_tranche);
                    Ok(())
                })?;
                return Ok(node_deployments);
            }
        }

        Ok(vec![vec![]])
    }
}

impl Handler<CreateDeploymentSchedule> for Scheduler {
    type Result = Result<Vec<Vec<(String, String, Node)>>>;

    fn handle(&mut self, message: CreateDeploymentSchedule, _: &mut Self::Context) -> Self::Result {
        message.generate()
    }
}
