use crate::{
    errors::RangerError,
    models::{helpers::uuid::Uuid, Score},
    services::database::{
        deployment::GetDeployment,
        score::{GetScoresByDeployment, GetScoresByMetric},
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use anyhow::Result;
use log::error;
use sdl_parser::{
    evaluation::Evaluation, parse_sdl, training_learning_objective::TrainingLearningObjectives,
};

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/tlo")]
pub async fn get_exercise_deployment_tlos(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Option<TrainingLearningObjectives>>, RangerError> {
    let (_, deployment_uuid) = path_variables.into_inner();
    let deployment = app_state
        .database_address
        .send(GetDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment"))?;
    let scenario = parse_sdl(&deployment.sdl_schema).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;

    Ok(Json(scenario.tlos))
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/tlo/{tlo_name}/evaluation")]
pub async fn get_exercise_deployment_tlo_evaluation(
    path_variables: Path<(Uuid, Uuid, String)>,
    app_state: Data<AppState>,
) -> Result<Json<Option<Evaluation>>, RangerError> {
    let (_, deployment_uuid, tlo_name) = path_variables.into_inner();
    let deployment = app_state
        .database_address
        .send(GetDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment"))?;
    let scenario = parse_sdl(&deployment.sdl_schema).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;

    let tlos = scenario.tlos.unwrap_or_default();
    if let Some(tlo) = tlos.get(&tlo_name) {
        if let Some(evaluations) = scenario.evaluations {
            let evaluation_return = evaluations.get(&tlo.evaluation).cloned();
            return Ok(Json(evaluation_return));
        }
    }

    Ok(Json(None))
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/score")]
pub async fn get_exercise_deployment_scores(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<Score>>, RangerError> {
    let (_, deployment_uuid) = path_variables.into_inner();

    let scores = app_state
        .database_address
        .send(GetScoresByDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get scores"))?;

    Ok(Json(scores))
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/tlo/{tlo_name}/evaluation/{metric_name}/score")]
pub async fn get_exercise_deployment_tlo_evaluation_metric_scores(
    path_variables: Path<(Uuid, Uuid, String, String)>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<Score>>, RangerError> {
    let (_, deployment_uuid, tlo_name, metric_name) = path_variables.into_inner();

    let scores = app_state
        .database_address
        .send(GetScoresByMetric(deployment_uuid, tlo_name, metric_name))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get scores"))?;

    Ok(Json(scores))
}
