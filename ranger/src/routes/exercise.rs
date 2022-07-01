use crate::{
    database::{AddScenario, GetScenario},
    errors::{RangerError, ServerResponseError},
    machiner::{CreateDeployment, DeploymentManager},
    AppState,
};
use actix::Actor;
use actix_web::{
    post,
    web::{Data, Path, Query},
    Error, HttpResponse,
};

use log::{error, info};
use sdl_parser::parse_sdl;
use serde::Deserialize;

#[post("exercise")]
pub async fn add_exercise(text: String, app_state: Data<AppState>) -> HttpResponse {
    match parse_sdl(&text) {
        Ok(schema) => {
            if let Err(error) = app_state
                .database_address
                .send(AddScenario(schema.scenario))
                .await
            {
                error!("Database actor mailbox error: {}", error);
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().body("Ok")
        }
        Err(error) => {
            error!("Failed to parse SDL: {}", error);
            HttpResponse::BadRequest().finish()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeploymentGroupNameQuery {
    name: Option<String>,
}

#[post("exercise/{scenario_name}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    query_param: Query<DeploymentGroupNameQuery>,
) -> Result<HttpResponse, Error> {
    let scenario_name = path_variables.into_inner();
    info!("Deploying scenario: {}", scenario_name);
    let scenario = app_state
        .database_address
        .send(GetScenario(scenario_name))
        .await
        .map_err(|error| {
            error!("Database actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Scenario not found {}", error);
            ServerResponseError(RangerError::ScenarioNotFound.into())
        })?;

    let deployment_group_name = query_param
        .into_inner()
        .name
        .unwrap_or_else(|| "default".to_string());

    info!("Using deplyoment group: {}", deployment_group_name);
    let deployment_address = DeploymentManager::new(
        app_state.deployer_actor_address.clone(),
        deployment_group_name,
    )
    .await
    .map_err(|error| {
        error!("DepoloyerGroup actor error: {}", error);
        ServerResponseError(RangerError::ActixMailBoxError.into())
    })?
    .start();

    deployment_address
        .send(CreateDeployment(scenario))
        .await
        .map_err(|error| {
            error!("DeployerGroup actor error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Failed to deploy scenario: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    Ok(HttpResponse::Ok().body("Ok"))
}
