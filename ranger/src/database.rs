use crate::{
    machiner::{
        CreateDeployment, DeploymentManager, FindDeploymentGroupByName, NodeDeploymentTrait,
    },
    templater::{filter_templation_results, separate_node_deployments_by_type, Templation},
};

use actix::{Actor, Addr, Context, Handler, Message};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::info;
use ranger_grpc::NodeDeployment;
use sdl_parser::Scenario;
use std::collections::{hash_map::Entry::Vacant, HashMap};
use uuid::Uuid;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct AddScenario(pub Scenario);

#[derive(Message, Debug)]
#[rtype(result = "Result<Scenario>")]
pub struct GetScenario(pub String);

#[derive(Default, PartialEq)]
pub struct Database {
    scenarios: HashMap<String, Scenario>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
pub trait Deployer {
    async fn deploy(
        self,
        deployment_manager_address: Addr<DeploymentManager>,
        requested_deployer_group_name: String,
    ) -> Result<Uuid>;
}
#[async_trait]
impl Deployer for Scenario {
    async fn deploy(
        self,
        deployment_manager_address: Addr<DeploymentManager>,
        requested_deployer_group_name: String,
    ) -> Result<Uuid> {
        let requested_deployment_group = deployment_manager_address
            .send(FindDeploymentGroupByName(requested_deployer_group_name))
            .await??;
        info!("Using deployment group: {}", requested_deployment_group.0);

        let deployment_id = Uuid::new_v4();
        let exercise_name = format!("{}-{}", &self.name, deployment_id);
        let templation_results = self
            .template_nodes(&requested_deployment_group.1.templaters)
            .await?;
        let template_ids = filter_templation_results(templation_results);

        let nodes = self
            .clone()
            .nodes
            .ok_or_else(|| anyhow!("No nodes found"))?;
        let node_deployments = NodeDeployment::default().create_from_nodes(
            nodes,
            template_ids,
            exercise_name.as_str(),
        )?;
        let node_deployments = separate_node_deployments_by_type(node_deployments)?;
        let vm_deployments = node_deployments.0;
        let switcher_deployments = node_deployments.1;
        
        let simulated_scheduler_output = vec![(vm_deployments, switcher_deployments)];
        let deployment_uuid = deployment_manager_address
            .send(CreateDeployment(
                simulated_scheduler_output,
                requested_deployment_group.1,
                exercise_name,
                deployment_id,
            ))
            .await??;
        Ok(deployment_uuid)
    }
}

impl Actor for Database {
    type Context = Context<Self>;
}

impl Handler<AddScenario> for Database {
    type Result = ();

    fn handle(&mut self, msg: AddScenario, _: &mut Context<Self>) -> Self::Result {
        if let Vacant(e) = self.scenarios.entry(msg.0.name.clone()) {
            e.insert(msg.0);
        } else {
            log::error!("This scenario already exists in the database");
        }
    }
}

impl Handler<GetScenario> for Database {
    type Result = Result<Scenario>;

    fn handle(&mut self, msg: GetScenario, _: &mut Context<Self>) -> Self::Result {
        match self.scenarios.get(&msg.0) {
            Some(scenario) => Ok(scenario.to_owned()),
            None => Err(anyhow!("Scenario not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{AddScenario, Database, GetScenario};
    use actix::{Actor, System};
    use anyhow::Result;
    use sdl_parser::test::TEST_SCHEMA;

    #[test]
    fn add_test_exercise() -> Result<()> {
        let system = System::new();
        system.block_on(async {
            let database_address = Database::new().start();
            let result = database_address
                .send(AddScenario(TEST_SCHEMA.scenario.clone()))
                .await;
            assert!(result.is_ok());
        });
        Ok(())
    }

    #[test]
    fn get_test_exercise() -> Result<()> {
        let system = System::new();
        let result = system.block_on(async {
            let database_address = Database::new().start();
            database_address
                .send(AddScenario(TEST_SCHEMA.scenario.clone()))
                .await
                .unwrap();
            let result = database_address
                .send(GetScenario("test-scenario".to_string()))
                .await;
            result?
        })?;
        assert_eq!(TEST_SCHEMA.scenario, result);
        Ok(())
    }
}
