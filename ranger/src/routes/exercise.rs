use crate::{
    database::{AddScenario, GetScenario},
    errors::RangerError,
    exercise::{AddExercise, Exercise, GetExercise},
    services::deployment::CreateDeployment,
    utilities::default_uuid,
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use anyhow::Result;
use log::{error, info};
use sdl_parser::parse_sdl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[post("exercise")]
pub async fn add_exercise(
    app_state: Data<AppState>,
    exercise: Json<Exercise>,
) -> Result<Json<Exercise>, ServerResponseError> {
    let exercise = exercise.into_inner();
    if let Err(error) = app_state
        .database_address
        .send(AddExercise(exercise.clone()))
        .await
    {
        error!("Database actor mailbox error: {}", error);
        return Err(ServerResponseError(RangerError::ActixMailBoxError.into()));
    }
    Ok(Json(exercise))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(default = "default_uuid")]
    pub id: Uuid,
    pub name: String,
    pub deployment_group: Option<String>,
}

#[post("exercise/{exercise_uuid}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    deployment: Json<Deployment>,
) -> Result<Json<Deployment>, RangerError> {
    let exercise_uuid = path_variables.into_inner();
    log::info!("Adding exercise: {}", exercise_uuid);
    let parsed_uuid = Uuid::parse_str(&exercise_uuid).unwrap();
    let exercise = app_state
        .database_address
        .send(GetExercise(parsed_uuid))
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
