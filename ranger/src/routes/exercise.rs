use crate::{
    database::{AddScenario, GetScenario},
    errors::RangerError,
    services::deployment::CreateDeployment,
    utilities::default_uuid,
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::Result;
use log::{error, info};
use sdl_parser::parse_sdl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(default = "default_uuid")]
    pub id: Uuid,
    pub name: String,
    pub deployment_group: Option<String>,
}

#[post("exercise/{scenario_name}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    deployment: Json<Deployment>,
) -> Result<Json<Deployment>, RangerError> {
    let deployment = deployment.into_inner();
    let exercise_name = path_variables.into_inner();
    info!("Deploying scenario: {exercise_name}");
    let scenario = app_state
        .database_address
        .send(GetScenario(exercise_name.clone()))
        .await
        .map_err(|error| {
            error!("Database actor mailbox error: {error}");
            RangerError::ActixMailBoxError
        })?
        .map_err(|_| {
            error!("Scenario not found");
            RangerError::ScenarioNotFound
        })?;

    app_state
        .deployment_manager_address
        .send(CreateDeployment(
            scenario,
            deployment.clone(),
            exercise_name.to_string(),
        ))
        .await
        .map_err(|error| {
            error!("Deployment manager actor mailbox error: {error}");
            RangerError::ActixMailBoxError
        })?
        .map_err(|error| {
            error!("Deployment error: {error}");
            RangerError::DeploymentFailed
        })?;

    Ok(Json(deployment))
}
