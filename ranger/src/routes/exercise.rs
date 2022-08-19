use crate::{
    database::{AddScenario, GetScenario},
    errors::{RangerError, ServerResponseError},
    services::deployment::CreateDeployment,
    AppState,
};
use actix_web::{
    post,
    web::{Data, Path, Query},
    Error, HttpResponse,
};
use anyhow::Result;
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
fn default_deployment_group_name_query() -> String {
    "default".to_string()
}
#[derive(Debug, Deserialize)]
pub struct DeploymentGroupNameQuery {
    #[serde(default = "default_deployment_group_name_query")]
    deployment_group: String,
}

#[post("exercise/{scenario_name}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    name_query: Query<DeploymentGroupNameQuery>,
) -> Result<HttpResponse, Error> {
    let scenario_name = path_variables.into_inner();
    info!("Deploying scenario: {scenario_name}");
    let scenario = app_state
        .database_address
        .send(GetScenario(scenario_name))
        .await
        .map_err(|error| {
            error!("Database actor mailbox error: {error}");
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|_| {
            error!("Scenario not found");
            ServerResponseError(RangerError::ScenarioNotFound.into())
        })?;
    let requested_deployer_group_name = name_query.into_inner().deployment_group;
    let deployment_uuid = app_state
        .deployment_manager_address
        .send(CreateDeployment(requested_deployer_group_name, scenario))
        .await
        .map_err(|error| {
            error!("Deployment manager actor mailbox error: {error}");
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Deployment error: {error}");
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;

    Ok(HttpResponse::Ok().body(format!("{deployment_uuid}")))
}
