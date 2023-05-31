use crate::{
    errors::RangerError,
    middleware::{authentication::UserInfo, deployment::DeploymentInfo},
    models::{
        helpers::uuid::Uuid,
        user::{User, UserAccount},
        Deployment, DeploymentElement, Exercise, NewDeployment, NewDeploymentResource, NewExercise,
        NewParticipant, NewParticipantResource, Participant, Score, UpdateExercise,
    },
    roles::RangerRole,
    services::{
        database::{
            account::GetAccount,
            condition::GetConditionMessagesByDeploymentId,
            deployment::{
                CreateDeployment, DeleteDeployment, GetDeployment,
                GetDeploymentElementByDeploymentId,
                GetDeploymentElementByDeploymentIdByScenarioReference, GetDeployments,
            },
            exercise::{CreateExercise, DeleteExercise, GetExercise, GetExercises},
            participant::{CreateParticipant, DeleteParticipant, GetParticipants},
        },
        deployment::{RemoveDeployment, StartDeployment},
        websocket::ExerciseWebsocket,
    },
    utilities::{
        create_database_error_handler, create_mailbox_error_handler,
        scenario::filter_node_roles_by_entity, try_some, Validation,
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
use futures::future::try_join_all;
use log::{error, info};
use ranger_grpc::capabilities::DeployerTypes as GrpcDeployerTypes;
use sdl_parser::{parse_sdl, Scenario};
use std::collections::HashMap;
use toml::value::Value;
use toml_query::read::TomlValueReadExt;

#[post("")]
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

#[get("")]
pub async fn get_exercises(app_state: Data<AppState>) -> Result<Json<Vec<Exercise>>, RangerError> {
    let exercises = app_state
        .database_address
        .send(GetExercises)
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get exercises"))?;

    Ok(Json(exercises))
}

#[put("")]
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
            update_exercise.clone(),
        ))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Update exercise"))?;
    log::debug!("Updated exercise: {}", exercise_uuid);

    Ok(Json(exercise))
}

#[get("")]
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

#[delete("")]
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

#[get("")]
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

#[post("")]
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

#[get("")]
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

#[delete("")]
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

#[get("deployment_element")]
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

#[get("websocket")]
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

#[post("participant")]
pub async fn add_participant(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
    participant_resource: Json<NewParticipantResource>,
) -> Result<Json<Participant>, RangerError> {
    let (_, deployment_uuid) = path_variables.into_inner();
    let participant_resource = participant_resource.into_inner();
    let deployment = app_state
        .database_address
        .send(GetDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment"))?;

    let parsed_sdl: Value = serde_yaml::from_str(&deployment.sdl_schema).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;
    let selector = format!("entities.{}", participant_resource.selector);
    let entity_option = parsed_sdl.read(&selector).map_err(|error| {
        error!("Failed to read entities from sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;
    if entity_option.is_none() {
        return Err(RangerError::EntityNotFound);
    }

    let new_participant = NewParticipant {
        id: Uuid::random(),
        deployment_id: deployment.id,
        selector: participant_resource.selector,
        user_id: participant_resource.user_id,
    };

    let participant = app_state
        .database_address
        .send(CreateParticipant(new_participant))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create participant"))?;

    Ok(Json(participant))
}

#[get("participant")]
pub async fn get_participants(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<Participant>>, RangerError> {
    let (_, deployment_uuid) = path_variables.into_inner();
    let participants = app_state
        .database_address
        .send(GetParticipants(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get participants"))?;
    Ok(Json(participants))
}

#[delete("participant/{participant_uuid}")]
pub async fn delete_participant(
    path_variables: Path<(Uuid, Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<String, RangerError> {
    let (_, _, participant_uuid) = path_variables.into_inner();
    app_state
        .database_address
        .send(DeleteParticipant(participant_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Delete deployments"))?;
    Ok(participant_uuid.to_string())
}

#[get("score")]
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
            if let Some((metric_key, metric)) = metrics.iter().find(|(_, metric)| {
                metric
                    .condition
                    .eq(&Some(condition_message.clone().condition_name))
            }) {
                if let Some(vm_name) =
                    vm_deplyoment_elements.get(&condition_message.virtual_machine_id.to_string())
                {
                    let metric_reference = match &metric.name {
                        Some(metric_name) => metric_name.to_owned(),
                        None => metric_key.to_owned(),
                    };

                    scores.push(Score::new(
                        exercise_uuid,
                        deployment_uuid,
                        metric_reference,
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

#[get("users")]
pub async fn get_exercise_deployment_users(
    app_state: Data<AppState>,
    user_details: UserInfo,
    deployment: DeploymentInfo,
) -> Result<Json<Vec<User>>, RangerError> {
    let scenario = parse_sdl(&deployment.sdl_schema).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;

    if let Some(scenario_nodes) = scenario.nodes {
        let requesters_nodes = match user_details.role {
            RangerRole::Admin => scenario_nodes,
            RangerRole::Participant => {
                let participants = app_state
                    .database_address
                    .send(GetParticipants(deployment.id))
                    .await
                    .map_err(create_mailbox_error_handler("Database"))?
                    .map_err(create_database_error_handler("Get participants"))?;
                let participant = participants
                    .into_iter()
                    .filter_map(
                        |participant| match participant.user_id.eq(&user_details.id) {
                            true => Some(participant),
                            false => None,
                        },
                    )
                    .last()
                    .ok_or(RangerError::DatabaseRecordNotFound)?;
                let selector = participant.selector.replace("entities.", "");

                scenario_nodes.into_iter().fold(
                    HashMap::new(),
                    |mut node_accumulator, (node_name, mut node)| {
                        if let Some(node_roles) = node.roles {
                            let filtered_node_roles =
                                filter_node_roles_by_entity(node_roles, selector.as_str());

                            if !filtered_node_roles.is_empty() {
                                node.roles = Some(filtered_node_roles);
                                node_accumulator.insert(node_name, node.clone());
                            }
                        }

                        node_accumulator
                    },
                )
            }
        };

        let roles_by_node = try_join_all(requesters_nodes.into_iter().map(|node| async {
            let source = try_some(node.1.source, "Node has no source")?;
            let sdl_roles = try_some(node.1.roles, "Node has no roles")?;
            let roles = sdl_roles.into_iter().map(|role| role.1).collect::<Vec<_>>();
            let template_deployment_element = app_state
                .database_address
                .send(GetDeploymentElementByDeploymentIdByScenarioReference(
                    deployment.id,
                    Box::new(source.clone()),
                    false,
                ))
                .await??;
            let vm_deployment_element = app_state
                .database_address
                .send(GetDeploymentElementByDeploymentIdByScenarioReference(
                    deployment.id,
                    Box::new(node.0),
                    false,
                ))
                .await??;

            let template_id_result = try_some(
                template_deployment_element.handler_reference,
                "Deployment element missing template id",
            )?;
            let vm_id_result = try_some(
                vm_deployment_element.handler_reference,
                "Deployment element missing vm id",
            )?;
            let template_id = Uuid::try_from(template_id_result.as_str())?;
            let vm_id = Uuid::try_from(vm_id_result.as_str())?;
            Ok((template_id, vm_id, roles))
        }))
        .await
        .map_err(create_database_error_handler(
            "Error getting node credentials",
        ))?;

        let users = try_join_all(
            roles_by_node
                .iter()
                .map(|(template_id, vm_id, roles)| async {
                    let accounts = try_join_all(roles.iter().map(|role| async {
                        let template_account: UserAccount = app_state
                            .database_address
                            .clone()
                            .send(GetAccount(*template_id, role.username.to_owned()))
                            .await??
                            .into();
                        Ok(template_account)
                    }))
                    .await
                    .map_err(create_database_error_handler(
                        "Error getting account information",
                    ))?;

                    Ok(User {
                        vm_id: *vm_id,
                        accounts,
                    })
                })
                .collect::<Vec<_>>(),
        )
        .await
        .map_err(create_database_error_handler(
            "Error gathering account information",
        ))?;
        Ok(Json(users))
    } else {
        Ok(Json(vec![]))
    }
}
