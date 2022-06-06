use actix_http::{body::BoxBody, StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use anyhow::Error;
use log::error;
use std::fmt::{Display, Formatter, Result};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum RangerServerError {
    #[error("Internal server error.")]
    InternalServerError,
    #[error("Can't get scenario from database.")]
    ScenarioNotFound,
    #[error("Can't get deployment from database.")]
    DeploymentNotFound,

}

#[derive(Debug)]
pub struct ServerResponseError(pub(crate) Error);

impl Display for ServerResponseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self.0)
    }
}

impl ResponseError for ServerResponseError {
    fn status_code(&self) -> actix_http::StatusCode {
        if let Some(ranger_server_error) = self.0.root_cause().downcast_ref::<RangerServerError>()
        {
            return match ranger_server_error {
                RangerServerError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
                RangerServerError::ScenarioNotFound => StatusCode::INTERNAL_SERVER_ERROR,
                RangerServerError::DeploymentNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            };
        }
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::with_body(self.status_code(), format!("{}", self.0)).map_into_boxed_body()
    }
}

impl From<anyhow::Error> for ServerResponseError {
    fn from(error: anyhow::Error) -> ServerResponseError {
        ServerResponseError(error)
    }
}
