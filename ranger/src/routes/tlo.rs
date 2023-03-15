use crate::{
    errors::RangerError,
    models::helpers::{score::Score, uuid::Uuid},
    services::database::{
        condition::GetConditionMessagesByDeploymentId,
        deployment::{GetDeployment, GetDeploymentElementByDeploymentId},
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler, try_some},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use anyhow::Result;
use bigdecimal::BigDecimal;
use log::error;
use ranger_grpc::capabilities::DeployerTypes as GrpcDeployerTypes;
use sdl_parser::{
    evaluation::Evaluation, parse_sdl, training_learning_objective::TrainingLearningObjectives,
};
use std::collections::HashMap;

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

    match scenario.tlos {
        Some(tlos) => {
            if let Some(tlo) = tlos.get(&tlo_name) {
                if let Some(evaluations) = scenario.evaluations {
                    let evaluation_return = evaluations.get(&tlo.evaluation).cloned();
                    return Ok(Json(evaluation_return));
                }
            }
            Ok(Json(None))
        }
        None => Ok(Json(None)),
    }
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/score")]
pub async fn get_exercise_deployment_scores(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<Score>>, RangerError> {
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

    let deployment_elements = app_state
        .database_address
        .send(GetDeploymentElementByDeploymentId(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment elements"))?;

    let vm_deplyoment_elements = deployment_elements
        .iter()
        .filter(|element| {
            matches!(element.deployer_type.0, GrpcDeployerTypes::VirtualMachine)
                && element.handler_reference.is_some()
        })
        .map(|element| {
            let vm_id = try_some(
                element.handler_reference.to_owned(),
                "VM element missing handler reference",
            )?;
            Ok((vm_id, element.scenario_reference.to_owned()))
        })
        .collect::<Result<HashMap<String, String>>>()
        .map_err(create_database_error_handler("Get deployment elements"))?;

    if let Some(metrics) = scenario.metrics {
        let mut scores: Vec<Score> = vec![];

        for condition_message in condition_messages.iter() {
            if let Some((metric_name, metric)) = metrics.iter().find(|(_, metric)| {
                metric
                    .condition
                    .eq(&Some(condition_message.clone().condition_name))
            }) {
                if let Some(vm_name) =
                    vm_deplyoment_elements.get(&condition_message.virtual_machine_id.to_string())
                {
                    scores.push(Score::new(
                        exercise_uuid,
                        deployment_uuid,
                        metric_name.to_owned(),
                        vm_name.to_owned(),
                        condition_message.virtual_machine_id,
                        condition_message.clone().value * BigDecimal::from(metric.max_score),
                        condition_message.created_at,
                    ))
                }
            }
        }

        return Ok(Json(scores));
    }
    Ok(Json(vec![]))
}
