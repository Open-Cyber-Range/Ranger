use crate::{
    database::{AddScenario, GetScenario},
    deployer::CreateDeployment,
    errors::{RangerError, ServerResponseError},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Path},
    Error, HttpResponse,
};

use log::error;
use sdl_parser::parse_sdl;

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

#[post("exercise/{scenario_name}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let scenario_name = path_variables.into_inner();
    log::info!("Adding scenario: {}", scenario_name);
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
    app_state
        .deployer_address
        .send(CreateDeployment(scenario))
        .await
        .map_err(|error| {
            error!("Database actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Failed to deploy scenario: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    Ok(HttpResponse::Ok().body("Ok"))
}
