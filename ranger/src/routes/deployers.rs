use crate::AppState;
use actix_web::web::Data;
use actix_web::{get, Error, HttpResponse};
use anyhow::Result;

#[get("deployer")]
pub async fn get_deployers(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let deployment_groups = app_state.configuration.deployment_groups.clone();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(deployment_groups))
}
