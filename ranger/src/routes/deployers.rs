use crate::deployers::GetDeployerGroups;
use crate::errors::{RangerError, ServerResponseError};
use crate::AppState;
use actix_web::web::Data;
use actix_web::{get, Error, HttpResponse};
use anyhow::Result;
use log::error;

#[get("deployers")]
pub async fn get_deployers(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let validated_deployer_groups = app_state
        .deployer_grouper_address
        .send(GetDeployerGroups)
        .await
        .map_err(|error| {
            error!("Deployer actor mailbox error: {}", error);
            ServerResponseError(RangerError::ActixMailBoxError.into())
        })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(validated_deployer_groups))
}
