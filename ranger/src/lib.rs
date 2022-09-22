pub mod configuration;
mod constants;
pub mod database;
pub mod errors;
pub mod models;
pub mod routes;
pub mod services;
pub(crate) mod utilities;

use crate::database::Database;
use actix::{Actor, Addr};
use configuration::Configuration;
use services::{
    deployer::DeployerDistribution, deployment::DeploymentManager, scheduler::Scheduler,
};

pub struct AppState {
    pub database_address: Addr<Database>,
    pub deployment_manager_address: Addr<DeploymentManager>,
    pub configuration: Configuration,
}

impl AppState {
    pub fn new(
        configuration: &configuration::Configuration,
        distributor: &Addr<DeployerDistribution>,
    ) -> Self {
        let schduler_address = Scheduler::new().start();
        AppState {
            database_address: Database::new().start(),
            deployment_manager_address: DeploymentManager::new(
                schduler_address,
                distributor.clone(),
                configuration.deployment_groups.clone(),
                configuration.default_deployment_group.to_string(),
            )
            .start(),
            configuration: configuration.to_owned(),
        }
    }
}
