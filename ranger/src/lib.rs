pub(crate) mod configuration;
mod constants;
pub mod errors;
pub mod models;
pub mod routes;
pub(crate) mod schema;
pub(crate) mod services;
pub(crate) mod utilities;

use crate::services::database::Database;
use actix::{Actor, Addr};
use anyhow::Result;
use configuration::{read_configuration, Configuration};
use log::{error, info};
use services::{
    deployer::{DeployerDistribution, DeployerFactory},
    deployment::DeploymentManager,
    mailer::Mailer,
    scheduler::Scheduler,
    websocket::WebSocketManager,
};

pub struct AppState {
    pub database_address: Addr<Database>,
    pub deployment_manager_address: Addr<DeploymentManager>,
    pub configuration: Configuration,
    pub websocket_manager_address: Addr<WebSocketManager>,
}

impl AppState {
    pub fn new(
        configuration: &configuration::Configuration,
        distributor: &Addr<DeployerDistribution>,
        database: &Addr<Database>,
        websocket_manager: &Addr<WebSocketManager>,
    ) -> Self {
        let schduler_address = Scheduler::new().start();
        AppState {
            database_address: database.clone(),
            deployment_manager_address: DeploymentManager::new(
                schduler_address,
                distributor.clone(),
                database.clone(),
                configuration.deployment_groups.clone(),
                configuration.default_deployment_group.to_string(),
            )
            .start(),
            configuration: configuration.to_owned(),
            websocket_manager_address: websocket_manager.clone(),
        }
    }
}

pub async fn app_setup(environment_arguments: Vec<String>) -> Result<(String, u16, AppState)> {
    let configuration = read_configuration(environment_arguments)?;
    let deployer_factory = DeployerFactory::new(&configuration.deployers)
        .await
        .unwrap_or_else(|error| panic!("Failed to create deployer distribution: {error}"))
        .start();

    let deployer_distributor = DeployerDistribution::new(
        deployer_factory,
        configuration.deployers.keys().cloned().collect(),
    )
    .await
    .unwrap_or_else(|error| panic!("Failed to create deployer distribution: {error}"))
    .start();

    let websocket_manager = WebSocketManager::new().start();
    let database = Database::try_new(&configuration.database_url, &websocket_manager)
        .unwrap_or_else(|error| {
            panic!(
                "Failed to create database connection to {} due to: {error}",
                &configuration.database_url
            )
        })
        .start();

    send_mail(configuration.clone());

    let app_state = AppState::new(
        &configuration,
        &deployer_distributor,
        &database,
        &websocket_manager,
    );
    Ok((configuration.host, configuration.port, app_state))
}

pub fn send_mail(configuration: Configuration) {
    if let Some(mailer_configuration) = configuration.mailer_configuration {
        match Mailer::send_mail(
            mailer_configuration,
            "real.person@real-email.com".to_string(),
        ) {
            Ok(_) => info!("Mail sent successfully!"),
            Err(e) => error!("Mailer failed: {:?}", e),
        }
    }
}
