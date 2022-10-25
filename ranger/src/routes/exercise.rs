use crate::{
    errors::RangerError,
    models::{
        helpers::uuid::Uuid, Deployment, Exercise, NewDeployment, NewExercise, UpdateExercise,
    },
    services::{
        database::{
            deployment::CreateDeployment,
            exercise::{CreateExercise, DeleteExercise, GetExercise},
        },
        deployment::StartDeployment,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler, Validation},
    AppState,
};
use actix_web::{
    delete, post, put,
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

#[put("exercise/{exercise_uuid}")]
pub async fn update_exercise(
    path_variables: Path<Uuid>,
    update_exercise: Json<UpdateExercise>,
    app_state: Data<AppState>,
) -> Result<Json<Exercise>, RangerError> {
    let exercise_uuid = path_variables.into_inner();
    let update_exercise = update_exercise.into_inner();
    let exercise = app_state
        .database_address
        .send(crate::services::database::exercise::UpdateExercise(
            exercise_uuid,
            update_exercise,
        ))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Update exercise"))?;

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
pub async fn add_exercise_deployment(
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

    let schema = Schema::from_yaml(&deployment.sdl_schema).map_err(|error| {
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
