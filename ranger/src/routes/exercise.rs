use crate::{
    errors::RangerError,
    models::{AddExercise, Deployment, Exercise, GetExercise},
    services::deployment::CreateDeployment,
    utilities::{create_mailbox_error_handler, Validation},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
};
use anyhow::Result;
use log::error;
use uuid::Uuid;

#[post("exercise")]
pub async fn add_exercise(
    app_state: Data<AppState>,
    exercise: Json<Exercise>,
) -> Result<Json<Exercise>, RangerError> {
    let exercise = exercise.into_inner();
    exercise.validate()?;
    if let Err(error) = app_state
        .database_address
        .send(AddExercise(exercise.clone()))
        .await
    {
        error!("Database actor mailbox error: {}", error);
        return Err(RangerError::ActixMailBoxError);
    }
    Ok(Json(exercise))
}

#[post("exercise/{exercise_uuid}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    deployment: Json<Deployment>,
) -> Result<Json<Deployment>, RangerError> {
    let deployment = deployment.into_inner();
    let exercise_uuid = path_variables.into_inner();

    let parsed_uuid = Uuid::parse_str(&exercise_uuid).unwrap();
    let exercise = app_state
        .database_address
        .send(GetExercise(parsed_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(|_| {
            error!("Scenario not found");
            RangerError::ScenarioNotFound
        })?;
    log::info!(
        "Adding deployment {} for exercise {}",
        exercise.name,
        deployment.name
    );

    app_state
        .deployment_manager_address
        .send(CreateDeployment(
            exercise.scenario,
            deployment.clone(),
            exercise.name,
        ))
        .await
        .map_err(create_mailbox_error_handler("Deployment manager"))?
        .map_err(|error| {
            error!("Deployment error: {error}");
            RangerError::DeploymentFailed
        })?;

    Ok(Json(deployment))
}
