use actix::{Actor, Handler, Message};
use anyhow::{anyhow, Ok, Result};
use ranger_grpc::NodeDeployment;
use sdl_parser::Scenario;

pub struct Scheduler;

impl Actor for Scheduler {
    type Context = actix::Context<Self>;
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler
    }
}

#[derive(Message, Debug, PartialEq)]
#[rtype(result = "Result<Vec<(Vec<NodeDeployment>, Vec<NodeDeployment>)>>")]
pub struct CreateDeploymentSchedule(Scenario);

impl CreateDeploymentSchedule {
    pub fn generate(&self) -> Result<Vec<(Vec<NodeDeployment>, Vec<NodeDeployment>)>> {
        let scenario = &self.0;
        let dependencies = scenario.get_dependencies()?;
        let tranches = dependencies.generate_tranches()?;

        if let Some(nodes) = &scenario.nodes {
            let deployment_schedule = tranches.iter().map(|tranche| {
                tranche.iter().map(|node_name| {
                    let nodes_value = nodes
                        .get(node_name)
                        .ok_or_else(|| anyhow!("Node not found"))?;
                    Ok(())
                })
            });
        }

        Ok(vec![(vec![], vec![])])
    }
}

impl Handler<CreateDeploymentSchedule> for Scheduler {
    type Result = Result<Vec<(Vec<NodeDeployment>, Vec<NodeDeployment>)>>;

    fn handle(&mut self, message: CreateDeploymentSchedule, _: &mut Self::Context) -> Self::Result {
        message.generate()
    }
}
