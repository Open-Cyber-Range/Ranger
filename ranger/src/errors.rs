use actix_http::{body::BoxBody, StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use anyhow::Error;
use std::fmt::{Display, Formatter, Result};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum RangerError {
    #[error("This scenario doesn't exist")]
    ScenarioNotFound,
    #[error("Internal server error")]
    ActixMailBoxError,
    #[error("Failed to deploy scenario")]
    DeploymentFailed,
    #[error("DeployerGroup not found")]
    DeployerGroupNotfound,
}

#[derive(Debug)]
pub struct ServerResponseError(pub Error);

impl Display for ServerResponseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self.0)
    }
}

impl ResponseError for ServerResponseError {
    fn status_code(&self) -> StatusCode {
        if let Some(ranger_server_error) = self.0.root_cause().downcast_ref::<RangerError>() {
            return match ranger_server_error {
                RangerError::ScenarioNotFound => StatusCode::NOT_FOUND,
                RangerError::DeployerGroupNotfound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
        }
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::with_body(self.status_code(), format!("{}", self.0)).map_into_boxed_body()
    }
}

impl From<Error> for ServerResponseError {
    fn from(error: Error) -> ServerResponseError {
        ServerResponseError(error)
    }
}
