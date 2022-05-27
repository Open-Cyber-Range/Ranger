pub mod configuration;
pub mod database;
pub mod node;
pub mod routes;
pub mod scenario;

use crate::database::Database;

pub struct AppState {
    pub database_address: actix::Addr<Database>,
}
