use crate::{
    errors::RangerError,
    models::{
        helpers::uuid::Uuid, Deployment, DeploymentElement, Exercise, NewDeployment,
        NewDeploymentResource, NewExercise, Score, UpdateExercise,
    },
    services::{
        database::{
            condition::GetConditionMessagesByDeploymentId,
            deployment::{
                CreateDeployment, DeleteDeployment, GetDeployment,
                GetDeploymentElementByDeploymentId, GetDeployments,
            },
            exercise::{CreateExercise, DeleteExercise, GetExercise, GetExercises},
        },
        deployment::{RemoveDeployment, StartDeployment},
        websocket::ExerciseWebsocket,
    },
    utilities::{
        create_database_error_handler, create_mailbox_error_handler, try_some, Validation,
    },
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Payload},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use anyhow::Result;
use bigdecimal::BigDecimal;
use log::{error, info};
use ranger_grpc::capabilities::DeployerTypes as GrpcDeployerTypes;
use sdl_parser::{
    node::Nodes,
    {parse_sdl, Scenario},
};
use std::collections::HashMap;

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
    log::debug!("Created exercise: {}", exercise.id);

    Ok(Json(exercise))
}

#[get("exercise")]
pub async fn get_exercises(app_state: Data<AppState>) -> Result<Json<Vec<Exercise>>, RangerError> {
    let exercises = app_state
        .database_address
        .send(GetExercises)
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get exercises"))?;

    Ok(Json(exercises))
}

#[put("exercise/{exercise_uuid}")]
pub async fn update_exercise(
    path_variables: Path<Uuid>,
    update_exercise: Json<UpdateExercise>,
    app_state: Data<AppState>,
) -> Result<Json<Exercise>, RangerError> {
    let exercise_uuid = path_variables.into_inner();
    let update_exercise = update_exercise.into_inner();

    if let Some(sdl) = &update_exercise.sdl_schema {
        if !sdl.is_empty() {
            Scenario::from_yaml(sdl).map_err(|error| {
                error!("SDL parsing error: {error}");
                RangerError::ScenarioParsingFailed
            })?;
        }
    }

    let exercise = app_state
        .database_address
        .send(crate::services::database::exercise::UpdateExercise(
            exercise_uuid,
            update_exercise.clone(),
        ))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Update exercise"))?;
    log::debug!("Updated exercise: {}", exercise_uuid);

    Ok(Json(exercise))
}

#[get("exercise/{exercise_uuid}")]
pub async fn get_exercise(
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
) -> Result<Json<Exercise>, RangerError> {
    let exercise_uuid = path_variables.into_inner();
    let exercise = app_state
        .database_address
        .send(crate::services::database::exercise::GetExercise(
            exercise_uuid,
        ))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get exercise"))?;
    log::debug!("Get exercise: {}", exercise_uuid);

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
    log::debug!("Deleted exercise {}", exercise_uuid);

    Ok(exercise_uuid.to_string())
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}")]
pub async fn get_exercise_deployment(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Deployment>, RangerError> {
    let (_, deployment_uuid) = path_variables.into_inner();
    let deployment = app_state
        .database_address
        .send(GetDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment"))?;

    Ok(Json(deployment))
}

#[post("exercise/{exercise_uuid}/deployment")]
pub async fn add_exercise_deployment(
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
    deployment_resource: Json<NewDeploymentResource>,
) -> Result<Json<Deployment>, RangerError> {
    let deployment_resource = deployment_resource.into_inner();
    let exercise_id = path_variables.into_inner();
    let exercise = app_state
        .database_address
        .send(GetExercise(exercise_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create exercise"))?;

    let scenario = Scenario::from_yaml(&deployment_resource.sdl_schema).map_err(|error| {
        error!("Deployment error: {error}");
        RangerError::DeploymentFailed
    })?;

    let deployment = NewDeployment::new(deployment_resource, exercise_id);

    log::debug!(
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
        .do_send(StartDeployment(scenario, deployment.clone(), exercise));

    Ok(Json(deployment))
}

#[delete("exercise/{exercise_uuid}/deployment/{deployment_uuid}")]
pub async fn delete_exercise_deployment(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<String, RangerError> {
    let (exercise_uuid, deployment_uuid) = path_variables.into_inner();
    let deployment = app_state
        .database_address
        .send(GetDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment"))?;
    app_state
        .deployment_manager_address
        .send(RemoveDeployment(exercise_uuid, deployment))
        .await
        .map_err(create_mailbox_error_handler("Deployment manager"))?
        .map_err(|error| {
            error!("Undeploying error: {error}");
            RangerError::UndeploymentFailed
        })?;

    info!("Deleting deployment {:?}", deployment_uuid.0);
    app_state
        .database_address
        .send(DeleteDeployment(deployment_uuid, false))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Delete deployment"))?;

    Ok(deployment_uuid.to_string())
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/deployment_element")]
pub async fn get_exercise_deployment_elements(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<DeploymentElement>>, RangerError> {
    let (_, deployment_uuid) = path_variables.into_inner();
    let deployment = app_state
        .database_address
        .send(GetDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment"))?;
    let elements = app_state
        .database_address
        .send(GetDeploymentElementByDeploymentId(deployment.id, false))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment elements"))?;

    Ok(Json(elements))
}

#[get("exercise/{exercise_uuid}/deployment")]
pub async fn get_exercise_deployments(
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<Deployment>>, RangerError> {
    let exercise_uuid = path_variables.into_inner();
    let deployments = app_state
        .database_address
        .send(GetDeployments(exercise_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployments"))?;

    Ok(Json(deployments))
}

#[get("exercise/{exercise_uuid}/websocket")]
pub async fn subscribe_to_exercise(
    req: HttpRequest,
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
    stream: Payload,
) -> Result<HttpResponse, Error> {
    let exercise_uuid = path_variables.into_inner();
    log::debug!("Subscribing websocket to exercise {}", exercise_uuid);
    let manager_address = app_state.websocket_manager_address.clone();
    let exercise_socket = ExerciseWebsocket::new(exercise_uuid, manager_address);

    ws::start(exercise_socket, &req, stream)
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/nodes")]
pub async fn get_exercise_deployment_nodes(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Option<Nodes>>, RangerError> {
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

    Ok(Json(scenario.nodes))
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/score")]
pub async fn get_exercise_deployment_scores(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<Score>>, RangerError> {
    let (exercise_uuid, deployment_uuid) = path_variables.into_inner();

    let mut condition_messages = app_state
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
        .send(GetDeploymentElementByDeploymentId(deployment_uuid, false))
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

        condition_messages.retain(|condition| {
            condition.created_at > scenario.start.naive_utc()
                && condition.created_at < scenario.end.naive_utc()
        });

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
