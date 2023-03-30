use crate::{
    errors::RangerError,
    models::{
        helpers::uuid::Uuid, Deployment, DeploymentElement, Exercise, NewDeployment,
        NewDeploymentResource, NewExercise, Score, UpdateExercise,
    },
    services::{
        database::{
            deployment::{
                CreateDeployment, DeleteDeployment, GetDeployment,
                GetDeploymentElementByDeploymentId, GetDeployments,
            },
            exercise::{CreateExercise, DeleteExercise, GetExercise, GetExercises},
            score::GetScores,
        },
        deployment::{RemoveDeployment, StartDeployment},
        websocket::ExerciseWebsocket,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler, Validation},
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Payload},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use anyhow::Result;
use log::{error, info};
use sdl_parser::{
    entity::Entities,
    evaluation::Evaluation,
    node::Nodes,
    training_learning_objective::TrainingLearningObjectives,
    {parse_sdl, Scenario},
};

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

#[post("exercise/{exercise_uuid}/deployment")]
pub async fn add_exercise_deployment(
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
    deployment_resource: Json<NewDeploymentResource>,
) -> Result<Json<Deployment>, RangerError> {
    let deployment_resource = deployment_resource.into_inner();
    let exercise_id = path_variables.into_inner();
    let deployment = NewDeployment::new(deployment_resource, exercise_id);

    let exercise = app_state
        .database_address
        .send(GetExercise(exercise_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create exercise"))?;

    let scenario = Scenario::from_yaml(&deployment.sdl_schema).map_err(|error| {
        error!("Deployment error: {error}");
        RangerError::DeploymentFailed
    })?;
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
        .send(DeleteDeployment(deployment_uuid))
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
        .send(GetDeploymentElementByDeploymentId(deployment.id))
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

#[get("/exercise/{exercise_uuid}/deployment/{deployment_uuid}/entities")]
pub async fn get_deployment_entities(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Option<Entities>>, RangerError> {
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
    let entities = scenario.entities;
    Ok(Json(entities))
}

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
            return Ok(Json(evaluations.get(&tlo.evaluation).cloned()));
        }
    }

    Ok(Json(None))
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/tlo/{tlo_name}/evaluation/{metric_name}/score")]
pub async fn get_exercise_deployment_tlo_evaluation_metric_scores(
    path_variables: Path<(Uuid, Uuid, String, String)>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<Score>>, RangerError> {
    let (_, deployment_uuid, tlo_name, metric_name) = path_variables.into_inner();

    let scores = app_state
        .database_address
        .send(GetScores(deployment_uuid, tlo_name, metric_name))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get scores"))?;

    Ok(Json(scores))
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
