use crate::{
    errors::RangerError,
    models::helpers::{score_element::ScoreElement, uuid::Uuid},
    services::database::{
        condition::GetConditionMessagesByDeploymentId, deployment::GetDeployment,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use anyhow::Result;
use bigdecimal::BigDecimal;
use log::error;
use sdl_parser::{
    evaluation::Evaluation, parse_sdl, training_learning_objective::TrainingLearningObjectives,
};
use std::collections::HashSet;

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
) -> Result<Json<Vec<ScoreElement>>, RangerError> {
    let (exercise_uuid, deployment_uuid) = path_variables.into_inner();

    let condition_messages = app_state
        .database_address
        .send(GetConditionMessagesByDeploymentId(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get condition_messages"))?;

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

    let metrics = scenario.metrics.unwrap_or_default();

    let mut score_elements = condition_messages
        .iter()
        .map(|condition_message| {
            let mut metric_name: String = Default::default();
            let mut score_multiplier: BigDecimal = Default::default();

            for (name, metric) in metrics.iter() {
                if metric
                    .condition
                    .eq(&Some(condition_message.clone().scenario_reference))
                {
                    metric_name = name.to_owned();
                    score_multiplier = metric.max_score.into();
                    break;
                }
            }

            ScoreElement::new(
                exercise_uuid,
                deployment_uuid,
                metric_name,
                condition_message.virtual_machine_id,
                condition_message.clone().value * score_multiplier,
                condition_message.created_at,
            )
        })
        .collect::<Vec<_>>();

    score_elements = ScoreElement::populate_vm_names(
        score_elements,
        app_state.database_address.clone(),
        deployment_uuid,
    )
    .await
    .map_err(|error| {
        error!("Failed to populate ScoreElements with VM names: {error}");
        RangerError::GenericInternalError
    })?;

    Ok(Json(score_elements))
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/tlo/{tlo_name}/evaluation/{metric_name}/score")]
pub async fn get_exercise_deployment_tlo_evaluation_metric_scores(
    path_variables: Path<(Uuid, Uuid, String, String)>,
    app_state: Data<AppState>,
) -> Result<Json<Option<Vec<ScoreElement>>>, RangerError> {
    let (exercise_uuid, deployment_uuid, _, req_metric_name) = path_variables.into_inner();

    let condition_messages = app_state
        .database_address
        .send(GetConditionMessagesByDeploymentId(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get condition_messages"))?;

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

    let score_elements_by_metric = ScoreElement::from_condition_messages_by_metric_name(
        exercise_uuid,
        deployment_uuid,
        scenario,
        condition_messages,
        req_metric_name,
    )
    .await;
    if let Some(score_elements) = score_elements_by_metric {
        let unique_vm_uuids: HashSet<Uuid> = score_elements
            .clone()
            .into_iter()
            .map(|element| element.vm_uuid)
            .collect();

        let score_elements = ScoreElement::populate_vm_names(
            score_elements,
            app_state.database_address.clone(),
            deployment_uuid,
        )
        .await
        .map_err(|error| {
            error!("Failed to populate ScoreElements with VM names: {error}");
            RangerError::GenericInternalError
        })?;

        let latest_unique_elements_by_vm_uuid: Option<Vec<ScoreElement>> = unique_vm_uuids
            .into_iter()
            .map(|vm_uuid| {
                score_elements
                    .clone()
                    .into_iter()
                    .filter_map(|element| element.vm_uuid.eq(&vm_uuid).then_some(Some(element)))
                    .collect::<Option<Vec<ScoreElement>>>()
                    .and_then(|score_elements| {
                        score_elements
                            .into_iter()
                            .max_by_key(|element| element.created_at)
                    })
            })
            .collect();

        return Ok(Json(latest_unique_elements_by_vm_uuid));
    }
    Ok(Json(None))
}
