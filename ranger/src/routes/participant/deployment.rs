use crate::{
    errors::RangerError,
    middleware::{
        authentication::UserInfo, deployment::DeploymentInfo, exercise::ExerciseInfo,
        keycloak::KeycloakInfo,
    },
    models::{helpers::uuid::Uuid, Deployment, DeploymentElement, ParticipantDeployment},
    services::database::deployment::{GetDeploymentElementByDeploymentId, GetDeployments},
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use futures_util::future::join_all;
use log::error;
use sdl_parser::parse_sdl;

#[get("")]
pub async fn get_participant_deployments(
    app_state: Data<AppState>,
    user_info: UserInfo,
    keycloak_info: KeycloakInfo,
    exercise: ExerciseInfo,
) -> Result<Json<Vec<ParticipantDeployment>>, RangerError> {
    let deployments = app_state
        .database_address
        .send(GetDeployments(exercise.id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployments"))?;

    let deployment_with_members: Vec<(Deployment, bool)> =
        join_all(deployments.into_iter().map(|deployment| async {
            let is_member = deployment
                .is_member(
                    user_info.id.clone(),
                    KeycloakInfo(keycloak_info.clone()),
                    app_state.configuration.keycloak.realm.clone(),
                )
                .await
                .unwrap_or(false);
            (deployment, is_member)
        }))
        .await;
    let deployments = deployment_with_members
        .into_iter()
        .filter_map(|(exercise, is_member)| {
            if is_member {
                return Some(exercise);
            }
            None
        })
        .collect::<Vec<_>>();

    let participant_deployments = deployments
        .into_iter()
        .map(ParticipantDeployment::from)
        .collect();

    Ok(Json(participant_deployments))
}

#[get("")]
pub async fn get_participant_deployment(
    deployment: DeploymentInfo,
) -> Result<Json<ParticipantDeployment>, RangerError> {
    let deployment = deployment.into_inner().into();

    Ok(Json(deployment))
}

#[get("")]
pub async fn get_participant_node_deployment_elements(
    path_variables: Path<(Uuid, Uuid, String)>,
    app_state: Data<AppState>,
    deployment: DeploymentInfo,
) -> Result<Json<Vec<DeploymentElement>>, RangerError> {
    let (_exercise_id, _deployment_id, entity_selector) = path_variables.into_inner();
    let deployment = deployment.into_inner();

    let elements = app_state
        .database_address
        .send(GetDeploymentElementByDeploymentId(deployment.id, false))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment elements"))?;

    let scenario = parse_sdl(&deployment.sdl_schema).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;

    let node_keys_filtered_by_entity: Vec<String> =
        scenario
            .nodes
            .into_iter()
            .fold(Vec::new(), |mut accumulator, nodes| {
                nodes.into_iter().for_each(|(node_key, node)| {
                    if let Some(roles) = node.roles {
                        roles.iter().for_each(|(_role_key, role)| {
                            if let Some(entities) = &role.entities {
                                if entities.contains(&entity_selector) {
                                    accumulator.push(node_key.clone());
                                }
                            }
                        })
                    }
                });
                accumulator
            });

    let entity_node_elements = elements
        .into_iter()
        .filter(|element| node_keys_filtered_by_entity.contains(&element.scenario_reference))
        .collect::<Vec<_>>();

    Ok(Json(entity_node_elements))
}
