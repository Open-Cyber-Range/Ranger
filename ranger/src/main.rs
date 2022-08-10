use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Error;
use ranger::configuration::read_configuration;
use ranger::deployers::DeployerGroups;
use ranger::routes::{
    basic::{status, version},
    deployers::get_deployers,
    exercise::{add_exercise, deploy_exercise},
};
use ranger::AppState;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let configuration = read_configuration(std::env::args().collect())?;
    let app_state = AppState::new();
    app_state
        .add_initial_deployergroups(
            configuration.deployment_groups,
            configuration.deployers,
            configuration.default_deployment_group,
        )
        .await?;
    DeployerGroups::start_all(
        app_state.deployer_grouper_address.clone(),
        app_state.deployment_manager_address.clone(),
    )
    .await?;

    let app_data = Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.to_owned())
            .service(status)
            .service(version)
            .service(
                scope("/api/v1")
                    .service(add_exercise)
                    .service(get_deployers)
                    .service(deploy_exercise),
            )
    })
    .bind((configuration.host, configuration.port))?
    .run()
    .await?;
    Ok(())
}
