use crate::{
    database::{AddScenario, GetScenario},
    errors::{RangerError, ServerResponseError},
    machiner::{CreateDeployment, FindDeploymentGroupByName, NodeDeploymentTrait},
    templater::{filter_templation_results, separate_node_deployments_by_type, Templation},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Path, Query},
    Error, HttpResponse,
};
use anyhow::{anyhow, Result};
use log::{error, info};
use ranger_grpc::NodeDeployment;
use sdl_parser::parse_sdl;
use serde::Deserialize;
use uuid::Uuid;

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
fn default_deployment_group_name_query() -> String {
    "default".to_string()
}
#[derive(Debug, Deserialize)]
pub struct DeploymentGroupNameQuery {
    #[serde(default = "default_deployment_group_name_query")]
    deployment_group: String,
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
    let requested_deployer_group_name = name_query.into_inner().deployment_group;

    let deployment_manager_address = app_state.deployment_manager_address.clone();

    let requested_deployment_group = deployment_manager_address
        .send(FindDeploymentGroupByName(requested_deployer_group_name))
        .await
        .map_err(|error| {
            error!("DeployerGroup actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("General error: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    info!("Using deployment group: {}", requested_deployment_group.0);

    let deployment_id = Uuid::new_v4();
    let exercise_name = format!("{}-{}", scenario.name, deployment_id);
    let exercise_name = exercise_name.as_str();
    let templation_results = scenario
        .clone()
        .template_nodes(&requested_deployment_group.1.templaters)
        .await
        .map_err(|error| {
            error!("General error: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    let template_ids = filter_templation_results(templation_results);

    let nodes = scenario
        .clone()
        .nodes
        .ok_or_else(|| anyhow!("No nodes found"))
        .map_err(|error| {
            error!("General error: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    let node_deployments = NodeDeployment::default()
        .create_from_nodes(nodes, template_ids, exercise_name)
        .map_err(|error| {
            error!("General error: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    let node_deployments =
        separate_node_deployments_by_type(node_deployments).map_err(|error| {
            error!("General error: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    let vm_deployments = node_deployments.0;
    let switcher_deployments = node_deployments.1;

    let simulated_scheduler_output = vec![(vm_deployments, switcher_deployments)];
    let deployment_uuid = app_state
        .deployment_manager_address
        .send(CreateDeployment(
            simulated_scheduler_output,
            requested_deployment_group.1,
            exercise_name.to_string(),
            deployment_id,
        ))
        .await
        .map_err(|error| {
            error!("Deployment manager actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?
        .map_err(|error| {
            error!("Deployment manager actor error: {}", error);
            ServerResponseError(RangerError::DeploymentFailed.into())
        })?;
    Ok(HttpResponse::Ok().body(format!("{deployment_uuid}")))
}
