use crate::{
    database::{AddScenario, GetScenario},
    deployers::get_deployer_groups,
    errors::{RangerError, ServerResponseError},
    machiner::CreateDeployment,
    templater::template_scenario,
    AppState,
};
use actix_web::{
    post,
    web::{Data, Path, Query},
    Error, HttpResponse,
};

use log::{error, info};
use sdl_parser::parse_sdl;
use serde::Deserialize;

#[post("exercise")]
pub async fn add_exercise(text: String, app_state: Data<AppState>) -> HttpResponse {
    match parse_sdl(&text) {
        Ok(schema) => {
            if let Err(error) = app_state
                .database_address
                .send(AddScenario(schema.scenario))
                .await
            {
                error!("Database actor mailbox error: {}", error);
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().body("Ok")
        }
        Err(error) => {
            error!("Failed to parse SDL: {}", error);
            HttpResponse::BadRequest().finish()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeploymentGroupNameQuery {
    deployment_group: Option<String>,
}

#[post("exercise/{scenario_name}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    name_query: Query<DeploymentGroupNameQuery>,
) -> Result<HttpResponse, Error> {
    let scenario_name = path_variables.into_inner();
    info!("Deploying scenario: {}", scenario_name);
    let scenario = app_state
        .database_address
        .send(GetScenario(scenario_name))
        .await
        .map_err(|error| {
            error!("Database actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Scenario not found {}", error);
            ServerResponseError(RangerError::ScenarioNotFound.into())
        })?;
    let requested_deployer_group_name = name_query
        .into_inner()
        .deployment_group
        .unwrap_or_else(|| "default".to_string());
    let deployer_groups = get_deployer_groups(app_state.deployer_grouper_address.clone()).await?;

    let requested_deployer_group = deployer_groups
        .find(requested_deployer_group_name.clone())
        .ok_or_else(|| {
            error!(
                "Deployment group not found: {}",
                requested_deployer_group_name
            );
            ServerResponseError(RangerError::DeployerGroupNotfound.into())
        })?;
    info!("Using deployment group: {}", requested_deployer_group.0);
    let deployment_group = requested_deployer_group.1.start().await;
    let templated_scenario = template_scenario(scenario, &deployment_group.templaters)
        .await
        .map_err(|error| {
            error!("Templater actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?;

    let deployment_uuid = app_state
        .deployment_manager_address
        .send(CreateDeployment(templated_scenario, deployment_group))
        .await
        .map_err(|error| {
            error!("Deployment manager actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?;
    Ok(HttpResponse::Ok().body(format!("{:?}", deployment_uuid)))
}
