use crate::{
    errors::RangerError,
    middleware::{authentication::UserInfo, deployment::DeploymentInfo},
    models::{helpers::uuid::Uuid, Event},
    services::database::{
        deployment::GetDeploymentElementByDeploymentId, event::GetEventsByDeploymentId,
        participant::GetParticipants,
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
use sdl_parser::parse_sdl;
use std::collections::HashMap;

#[get("entity/{entity_selector}/events")]
pub async fn get_participant_events(
    app_state: Data<AppState>,
    user_details: UserInfo,
    deployment: DeploymentInfo,
    path_variables: Path<(Uuid, Uuid, String)>,
) -> Result<Json<Vec<Event>>, RangerError> {
    let (_exercise_uuid, deployment_uuid, entity_selector) = path_variables.into_inner();
    let scenario = parse_sdl(&deployment.sdl_schema).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;

    let participants = app_state
        .database_address
        .send(GetParticipants(deployment.id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get participants"))?;
    let valid_participant_entity_selectors: Vec<String> = participants
        .into_iter()
        .filter_map(
            |participant| match participant.user_id.eq(&user_details.id) {
                true => Some(participant.selector),
                false => None,
            },
        )
        .collect();
    if valid_participant_entity_selectors.contains(&entity_selector) {
        let nodes_by_entity =
            scenario
                .nodes
                .into_iter()
                .fold(HashMap::new(), |mut accumulator, nodes| {
                    nodes.into_iter().for_each(|(node_key, node)| {
                        if let Some(roles) = node.roles.clone() {
                            roles.iter().for_each(|(_role_key, role)| {
                                if let Some(entities) = &role.entities {
                                    if entities.contains(&entity_selector) {
                                        accumulator.insert(node_key.clone(), node.clone());
                                    }
                                }
                            })
                        }
                    });
                    accumulator
                });
        let deployment_elements = app_state
            .database_address
            .send(GetDeploymentElementByDeploymentId(deployment.id, false))
            .await
            .map_err(create_mailbox_error_handler("Database"))?
            .map_err(create_database_error_handler("Get deployment elements"))?;
        let entity_deployment_elements = deployment_elements
            .into_iter()
            .filter_map(
                |element| match nodes_by_entity.contains_key(&element.scenario_reference) {
                    true => Some(element),
                    false => None,
                },
            )
            .collect::<Vec<_>>();
        let deployment_events = app_state
            .database_address
            .send(GetEventsByDeploymentId(deployment_uuid))
            .await
            .map_err(create_mailbox_error_handler("Database"))?
            .map_err(create_database_error_handler("Get events"))?;
        let entity_events = deployment_events
            .into_iter()
            .fold(vec![], |mut accumulator, event| {
                entity_deployment_elements.iter().for_each(|element| {
                    if let Some(handler_reference) = &element.handler_reference {
                        if event.parent_node_id.to_string().eq(handler_reference) {
                            accumulator.push(event.clone());
                        }
                    }
                });

                accumulator
            });
        let triggered_entity_events = entity_events
            .into_iter()
            .filter(|event| event.has_triggered)
            .collect::<Vec<_>>();

        Ok(Json(triggered_entity_events))
    } else {
        error!("Requested Entity is not among the Participants Entities");
        Err(RangerError::NotAuthorized)
    }
}
