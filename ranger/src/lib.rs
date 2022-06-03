pub mod configuration;
pub mod database;
pub mod node;
pub mod routes;
pub mod scenario;
pub mod deployer;

use crate::database::Database;
use actix::Addr;
use deployer::DeploymentManager;

pub struct AppState {
    pub database_address: Addr<Database>,
    pub deployer_address: Addr<DeploymentManager>,
}
