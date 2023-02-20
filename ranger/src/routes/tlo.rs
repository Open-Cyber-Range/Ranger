use crate::{
    errors::RangerError,
    models::helpers::{score_element::ScoreElement, uuid::Uuid},
    services::database::{
        condition::GetConditionMessagesByDeploymentId,
        deployment::{GetDeployment, GetDeploymentElementByDeploymentIdByHandlerReference},
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
use std::collections::{HashMap, HashSet};

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

    let score_elements = condition_messages
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
                condition_message.virtual_machine_id.to_string(),
                condition_message.clone().value * score_multiplier,
                condition_message.created_at,
            )
        })
        .collect::<Vec<_>>();

    let unique_vm_uuids: HashSet<String> = score_elements
        .clone()
        .into_iter()
        .map(|element| element.vm_name)
        .collect();

    let mut vm_names_by_uuid: HashMap<String, String> = Default::default();

    for vm_uuid in unique_vm_uuids {
        let deployment_element = app_state
            .database_address
            .send(GetDeploymentElementByDeploymentIdByHandlerReference(
                deployment_uuid,
                vm_uuid.to_owned(),
            ))
            .await
            .map_err(create_mailbox_error_handler("Database"))?
            .map_err(create_database_error_handler("Get deployment element"))?;
        vm_names_by_uuid.insert(vm_uuid.to_owned(), deployment_element.scenario_reference);
    }

    let score_elements: Vec<ScoreElement> = score_elements
        .iter()
        .map(|element| ScoreElement {
            vm_name: vm_names_by_uuid
                .get(&element.vm_name)
                .unwrap_or(&element.vm_name)
                .to_string(),
            ..element.clone()
        })
        .collect();

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

    let results = ScoreElement::from_condition_messages_by_metric_name(
        exercise_uuid,
        deployment_uuid,
        scenario,
        condition_messages,
        req_metric_name,
    )
    .await;

    Ok(Json(results))
}
