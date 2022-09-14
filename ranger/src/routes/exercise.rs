use crate::{
    errors::{RangerError, ServerResponseError},
    exercise::{AddExercise, Exercise, GetExercise},
    machiner::{CreateDeployment, DeploymentManager},
    AppState,
};
use actix::Actor;
use actix_web::{
    post,
    web::{Data, Json, Path},
    Error, HttpResponse,
};
use anyhow::Result;
use log::error;
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

#[post("exercise/{exercise_uuid}/deployment")]
pub async fn deploy_exercise(
    app_state: Data<AppState>,
    path_variables: Path<String>,
) -> Result<HttpResponse, Error> {
    let exercise_uuid = path_variables.into_inner();
    log::info!("Adding exercise: {}", exercise_uuid);
    let parsed_uuid = Uuid::parse_str(&exercise_uuid).unwrap();
    let exercise = app_state
        .database_address
        .send(GetExercise(parsed_uuid))
        .await
        .map_err(|error| {
            error!("Database actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Scenario not found {}", error);
            ServerResponseError(RangerError::ScenarioNotFound.into())
        })?;

    let deployment_address = DeploymentManager::new(app_state.deployer_actor_address.clone())
        .await
        .map_err(|error| {
            error!("DeployerGroup actor error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .start();
    deployment_address
        .send(CreateDeployment(exercise.scenario.clone()))
        .await
        .map_err(|error| {
            error!("Database actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Failed to deploy scenario: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    Ok(HttpResponse::Ok().body(format!("{:?}", exercise)))
}
