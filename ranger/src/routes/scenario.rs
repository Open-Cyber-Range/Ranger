use crate::{
    errors::RangerError,
    middleware::authentication::UserInfo,
    models::helpers::uuid::Uuid,
    roles::RangerRole,
    services::database::{deployment::GetDeployment, participant::GetParticipants},
    utilities::{
        create_database_error_handler, create_mailbox_error_handler,
        scenario::{filter_scenario_by_role, flatten_entities},
        try_some,
    },
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use anyhow::Result;
use log::error;
use sdl_parser::{parse_sdl, Scenario};

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/scenario")]
pub async fn get_exercise_deployment_scenario(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
    user_details: UserInfo,
) -> Result<Json<Scenario>, RangerError> {
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

    match user_details.role {
        RangerRole::Admin => Ok(Json(scenario)),
        RangerRole::Participant => {
            let participants = app_state
                .database_address
                .send(GetParticipants(deployment_uuid))
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

            let normalized_selector = participant.selector.replace("entities.", "");
            let entities = try_some(scenario.entities.clone(), "Entities not found not found")
                .map_err(|error| {
                    error!("{error}");
                    RangerError::EntityNotFound
                })?;

            let flattened_entities = flatten_entities(entities);
            let participant_entity_result = flattened_entities.get(&normalized_selector);
            let participant_entity = try_some(participant_entity_result, "Entity not found")
                .map_err(|error| {
                    error!("{error}");
                    RangerError::EntityNotFound
                })?;

            let participant_role = try_some(participant_entity.role.clone(), "Entity missing role")
                .map_err(|error| {
                    error!("{error}");
                    RangerError::ScenarioParsingFailed
                })?;

            let participant_scenario = filter_scenario_by_role(&scenario, participant_role);
            Ok(Json(participant_scenario))
        }
    }
}
