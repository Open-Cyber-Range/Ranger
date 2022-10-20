use crate::{
    errors::RangerError,
    models::{helpers::uuid::Uuid, Deployment, Exercise, NewDeployment, NewExercise},
    services::{
        database::{
            deployment::CreateDeployment,
            exercise::{CreateExercise, DeleteExercise, GetExercise},
            scenario::GetScenario,
        },
        deployment::StartDeployment,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler, Validation},
    AppState,
};
use actix_web::{
    delete, post,
    web::{Data, Json, Path},
};
use anyhow::Result;
use log::error;
use sdl_parser::Schema;

#[post("exercise")]
pub async fn add_exercise(
    app_state: Data<AppState>,
    exercise: Json<NewExercise>,
) -> Result<Json<Exercise>, RangerError> {
    let exercise = exercise.into_inner();
    exercise.validate()?;

    let exercise = app_state
        .database_address
        .send(CreateExercise(exercise))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create exercise"))?;

    Ok(Json(exercise))
}

#[delete("exercise/{exercise_uuid}")]
pub async fn delete_exercise(
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
) -> Result<String, RangerError> {
    let exercise_uuid = path_variables.into_inner();
    app_state
        .database_address
        .send(DeleteExercise(exercise_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Delete exercise"))?;

    Ok(exercise_uuid.to_string())
}

#[post("exercise/{exercise_uuid}/deployment")]
pub async fn add_deployment(
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
    deployment: Json<NewDeployment>,
) -> Result<Json<Deployment>, RangerError> {
    let deployment = deployment.into_inner();
    let exercise_uuid = path_variables.into_inner();

    let exercise = app_state
        .database_address
        .send(GetExercise(exercise_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create exercise"))?;
    let scenario = app_state
        .database_address
        .send(GetScenario(exercise.scenario_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create exercise"))?;
    let schema = Schema::from_yaml(&scenario.content).map_err(|error| {
        error!("Deployment error: {error}");
        RangerError::DeploymentFailed
    })?;
    log::info!(
        "Adding deployment {} for exercise {}",
        exercise.name,
        deployment.name
    );
    let deployment = app_state
        .database_address
        .send(CreateDeployment(deployment))
        .await
        .map_err(create_mailbox_error_handler("Deployment"))?
        .map_err(create_database_error_handler("Create deployment"))?;

    app_state
        .deployment_manager_address
        .send(StartDeployment(
            schema.scenario,
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
