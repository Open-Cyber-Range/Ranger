use crate::{
    errors::RangerError,
    models::helpers::uuid::Uuid,
    services::database::deployment::GetDeployment,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
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

    Ok(Json(scenario))
}
