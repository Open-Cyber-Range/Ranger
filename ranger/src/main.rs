use actix::Actor;
use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Error;
use ranger::configuration::read_configuration;
use ranger::database::Database;
use ranger::deployer::DeploymentManager;
use ranger::node::NodeClient;
use ranger::routes::basic::{status, version};
use ranger::routes::exercise::{add_exercise, deploy_exercise};
use ranger::AppState;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let configuration = read_configuration(std::env::args().collect())?;
    let node_client_address = NodeClient::new(configuration.node_deployer_addresses[0].clone())
        .await?
        .start();
    let database_address = Database::new().start();
    let deployer_address = DeploymentManager::new(node_client_address.clone()).start();
    HttpServer::new(move || {
        let app_state = Data::new(AppState {
            database_address: database_address.clone(),
            deployer_address: deployer_address.clone(),
        });
        App::new()
            .app_data(app_state)
            .service(status)
            .service(version)
            .service(
                scope("/api/v1")
                    .service(add_exercise)
                    .service(deploy_exercise),
            )
    })
    .bind((configuration.host, configuration.port))?
    .run()
    .await?;
    Ok(())
}
