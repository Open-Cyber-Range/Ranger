use crate::{
    errors::RangerError,
    models::{helpers::uuid::Uuid, NewScenario, Scenario},
    services::database::scenario::{CreateScenario, DeleteScenario},
    utilities::{create_database_error_handler, create_mailbox_error_handler, Validation},
    AppState,
};
use actix_web::{
    delete, post,
    web::{Data, Json, Path},
};
use anyhow::Result;

#[post("scenario")]
pub async fn add_scenario(
    app_state: Data<AppState>,
    new_scenario: Json<NewScenario>,
) -> Result<Json<Scenario>, RangerError> {
    let new_scenario = new_scenario.into_inner();
    new_scenario.validate()?;

    let scenario = app_state
        .database_address
        .send(CreateScenario(new_scenario.clone()))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create scenario"))?;

    Ok(Json(scenario))
}

#[delete("scenario/{scenario_uuid}")]
pub async fn delete_scenario(
    app_state: Data<AppState>,
    path_variables: Path<Uuid>,
) -> Result<String, RangerError> {
    let scenario_uuid = path_variables.into_inner();
    app_state
        .database_address
        .send(DeleteScenario(scenario_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Delete scenario"))?;

    Ok(scenario_uuid.to_string())
}
