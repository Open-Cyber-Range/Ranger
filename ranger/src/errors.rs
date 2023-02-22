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
    #[error("Failed to undeploy exercise")]
    UndeploymentFailed,
    #[error("DeployerGroup not found")]
    DeployerGroupNotfound,
    #[error("Exercise name too long")]
    ExerciseNameTooLong,
    #[error("Deployment name too long")]
    DeploymentNameTooLong,
    #[error("Failed to parse uuid")]
    UuidParsingFailed,
    #[error("Failed to parse scenario")]
    ScenarioParsingFailed,
    #[error("Internal server error")]
    DatabaseUnexpected,
    #[error("Conflict")]
    DatabaseConflict,
    #[error("Not found")]
    DatabaseRecordNotFound,
    #[error("Mailer configuration not found")]
    MailerConfigurationNotFound,
    #[error("Failed to create email message")]
    EmailMessageCreationFailed,
}

impl ResponseError for RangerError {
    fn status_code(&self) -> StatusCode {
        match self {
            RangerError::ScenarioNotFound => StatusCode::NOT_FOUND,
            RangerError::DeployerGroupNotfound => StatusCode::NOT_FOUND,
            RangerError::ExerciseNameTooLong => StatusCode::UNPROCESSABLE_ENTITY,
            RangerError::DeploymentNameTooLong => StatusCode::UNPROCESSABLE_ENTITY,
            RangerError::UuidParsingFailed => StatusCode::UNPROCESSABLE_ENTITY,
            RangerError::ScenarioParsingFailed => StatusCode::UNPROCESSABLE_ENTITY,
            RangerError::DatabaseConflict => StatusCode::CONFLICT,
            RangerError::DatabaseRecordNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        error!("Error: {:?}", self);
        HttpResponse::with_body(self.status_code(), format!("{}", self)).map_into_boxed_body()
    }
}
