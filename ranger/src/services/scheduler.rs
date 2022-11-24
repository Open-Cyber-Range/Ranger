use actix::{Actor, Handler, Message};
use anyhow::{anyhow, Ok, Result};
use log::info;
use sdl_parser::{feature::Feature, infrastructure::InfraNode, node::Node, Scenario};

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

#[derive(Message, Debug, PartialEq, Eq)]
#[rtype(result = "Result<Vec<Vec<(String, Node, InfraNode)>>>")]
pub struct CreateDeploymentSchedule(pub(crate) Scenario);

impl CreateDeploymentSchedule {
    fn create_node_name(infra_node: &InfraNode, node_name: String, count: u32) -> String {
        match infra_node.count {
            0 | 1 => node_name,
            _ => format!("{}-{}", node_name, count),
        }
    }

    pub fn generate(&self) -> Result<Vec<Vec<(String, Node, InfraNode)>>> {
        let scenario = &self.0;
        let dependencies = scenario.get_node_dependencies()?;
        let tranches = dependencies.generate_tranches()?;

        if let Some(infrastructure) = &scenario.infrastructure {
            if let Some(nodes) = &scenario.nodes {
                let mut node_deployments: Vec<Vec<(String, Node, InfraNode)>> = Vec::new();
                tranches.iter().try_for_each(|tranche| {
                    let mut new_tranche = Vec::new();
                    tranche.iter().try_for_each(|node_name| {
                        if let Some(infra_value) = infrastructure.get(node_name) {
                            let node_value =
                                nodes.get(node_name).ok_or_else(|| anyhow!("Node value"))?;
                            for n in 0..infra_value.count {
                                new_tranche.push((
                                    Self::create_node_name(infra_value, node_name.clone(), n),
                                    node_value.clone(),
                                    infra_value.clone(),
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

#[derive(Message, Debug, PartialEq, Eq)]
#[rtype(result = "Result<Vec<Vec<(String, Feature)>>>")]
pub struct CreateFeatureDeploymentSchedule(pub(crate) Scenario, pub(crate) Node);

impl CreateFeatureDeploymentSchedule {
    pub fn generate(&self) -> Result<Vec<Vec<(String, Feature)>>> {
        let scenario = &self.0;
        let node = &self.1;

        //this creates a deployment order for ALL features in the scenario? i think?
        //what i need is a dependency tree of the current nodes features
        let dependencies = scenario.get_feature_dependencies()?;
        let tranches = dependencies.generate_tranches()?;

        if let Some(features) = &scenario.features {
            let mut feature_deployments: Vec<Vec<(String, Feature)>> = Vec::new();

            tranches.iter().try_for_each(|tranche| {
                let mut new_tranche = Vec::new();
                tranche.iter().try_for_each(|feature_name| {
                    let feature_value = features
                        .get(feature_name)
                        .ok_or_else(|| anyhow!("feature value"))?;
                    new_tranche.push((feature_name.clone(), feature_value.clone()));
                    Ok(())
                })?;
                feature_deployments.push(new_tranche);
                Ok(())
            })?;
            return Ok(feature_deployments);
        }

        Ok(vec![vec![]])
    }
}

impl Handler<CreateDeploymentSchedule> for Scheduler {
    type Result = Result<Vec<Vec<(String, Node, InfraNode)>>>;

    fn handle(&mut self, message: CreateDeploymentSchedule, _: &mut Self::Context) -> Self::Result {
        message.generate()
    }
}

impl Handler<CreateFeatureDeploymentSchedule> for Scheduler {
    type Result = Result<Vec<Vec<(String, Feature)>>>;

    fn handle(
        &mut self,
        message: CreateFeatureDeploymentSchedule,
        _: &mut Self::Context,
    ) -> Self::Result {
        message.generate()
    }
}
