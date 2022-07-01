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
    Error, HttpRequest, HttpResponse,
};

use log::error;
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
struct DeploymentGroupNameQuery {
    name: String,
}

#[post("exercise/{scenario_name}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let scenario_name = path_variables.into_inner();
    log::info!("Deploying scenario: {}", scenario_name);
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

    if !request.query_string().is_empty() {
        let deployment_group_name =
            Query::<DeploymentGroupNameQuery>::from_query(request.query_string())?
                .into_inner()
                .name;
        let deployment_address = DeploymentManager::new(
            app_state.deployer_actor_address.clone(),
            deployment_group_name,
        )
        .await
        .map_err(|error| {
            error!("DeployerGroup actor error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .start();

        deployment_address
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
        return Ok(HttpResponse::Ok().body("Ok"));
    }
    Ok(HttpResponse::Ok().body("Not Ok"))
}
