use std::collections::HashMap;

use crate::errors::RangerError;
use crate::models::helpers::grpc_package::SerializableGrpcPackage;
use crate::routes::get_query_param;
use crate::services::deployer::{DeputyPackageQueryByType, DeputyPackageQueryGetExercise};
use crate::services::deployment::GetDefaultDeployers;
use crate::utilities::{create_database_error_handler, create_mailbox_error_handler};
use crate::AppState;
use actix_web::web::{Data, Query};
use actix_web::{get, Error, HttpResponse};
use anyhow::Result;
use log::error;
use ranger_grpc::Source;
use sdl_parser::parse_sdl;

#[get("")]
pub async fn get_deputy_packages_by_type(
    app_state: Data<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let deployers = app_state
        .deployment_manager_address
        .send(GetDefaultDeployers())
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get default deployers"))?;

    let package_type = get_query_param(&params, "type")?;

    let query_result = app_state
        .deployer_distributor_address
        .send(DeputyPackageQueryByType(package_type, deployers))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("get packases"))?;

    let serializable_packages: Vec<SerializableGrpcPackage> =
        query_result.into_iter().map(Into::into).collect();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(serializable_packages))
}

#[get("")]
pub async fn get_exercise_by_source(
    app_state: Data<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let deployers = app_state
        .deployment_manager_address
        .send(GetDefaultDeployers())
        .await
        .map_err(create_mailbox_error_handler("Deptuy Query"))?
        .map_err(create_database_error_handler("Get default deployers"))?;

    let source = Source {
        name: get_query_param(&params, "name")?,
        version: get_query_param(&params, "version")?,
    };

    let query_result = app_state
        .deployer_distributor_address
        .send(DeputyPackageQueryGetExercise(source, deployers))
        .await
        .map_err(create_mailbox_error_handler("Deptuy Query"))?
        .map_err(create_database_error_handler("get exercise package"))?;

    let scenario = parse_sdl(&query_result).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(scenario))
}
