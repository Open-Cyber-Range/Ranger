use actix_http::{body::BoxBody, StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use log::error;
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

impl ResponseError for RangerError {
    fn status_code(&self) -> StatusCode {
        match self {
            RangerError::ScenarioNotFound => StatusCode::NOT_FOUND,
            RangerError::DeployerGroupNotfound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        error!("Error: {:?}", self);
        HttpResponse::with_body(self.status_code(), format!("{}", self)).map_into_boxed_body()
    }
}
