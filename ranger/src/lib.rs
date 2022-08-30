pub mod configuration;
pub mod database;
pub mod deployers;
pub mod errors;
pub mod node;
pub mod routes;
pub mod services;
pub mod templater;
pub mod utilities;

use crate::database::Database;
use actix::{Actor, Addr};
use anyhow::{anyhow, Result};
use deployers::{get_deployer_capabilities, AddDeployerGroups, DeployerGroups};
use services::{deployment::DeploymentManager, scheduler::Scheduler};

use std::collections::HashMap;

const DEFAULT_DEPLOYER_GROUP_NAME: &str = "default";
pub struct AppState {
    pub database_address: Addr<Database>,
    pub deployer_grouper_address: Addr<DeployerGroups>,
    pub deployment_manager_address: Addr<DeploymentManager>,
}

impl AppState {
    pub fn new() -> Self {
        let schduler_address = Scheduler::new().start();
        AppState {
            database_address: Database::new().start(),
            deployer_grouper_address: DeployerGroups::new().start(),
            deployment_manager_address: DeploymentManager::new(schduler_address).start(),
        }
    }

    pub async fn add_initial_deployergroups(
        &self,
        deployment_groups: HashMap<String, Vec<String>>,
        deployers: HashMap<String, String>,
        default_deployer_group: String,
    ) -> Result<()> {
        let mut deployer_groups = DeployerGroups::initialize_with_group_names(&deployment_groups);
        let mut deployers = get_deployer_capabilities(deployers).await?;

        deployers.iter_mut().for_each(|deployer| {
            deployment_groups.iter().for_each(|deployer_group| {
                if deployer_group.1.contains(&deployer.deployer_name) {
                    if let Some(deployer_group) = deployer_groups.0.get_mut(deployer_group.0) {
                        deployer_group.insert_by_capability(deployer);
                    }
                }
            });
        });
        let default_deployer_group_value = deployer_groups
            .0
            .get(&default_deployer_group)
            .ok_or_else(|| anyhow!("Default group with given name not found"))?
            .clone();

        deployer_groups.0.insert(
            DEFAULT_DEPLOYER_GROUP_NAME.to_string(),
            default_deployer_group_value,
        );

        self.deployer_grouper_address
            .send(AddDeployerGroups(deployer_groups))
            .await?;
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
