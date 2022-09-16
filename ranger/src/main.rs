use actix::Actor;
use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Error;
use ranger::configuration::read_configuration;
use ranger::routes::{
    basic::{status, version},
    exercise::{add_exercise, deploy_exercise},
};
use ranger::services::deployer::{DeployerDistribution, DeployerFactory};
use ranger::AppState;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let configuration = read_configuration(std::env::args().collect())?;
    let deployer_factory = DeployerFactory::new(&configuration.deployers)
        .await?
        .start();
    let deployer_distributor = DeployerDistribution::new(
        deployer_factory,
        configuration.deployers.keys().cloned().collect(),
    )
    .await?
    .start();
    let app_state = AppState::new(&configuration, &deployer_distributor);

    let app_data = Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.to_owned())
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
