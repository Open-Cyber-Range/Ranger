use crate::{
    errors::RangerError,
    middleware::exercise::ExerciseInfo,
    models::{helpers::uuid::Uuid, Email, EmailResource, NewEmail},
    services::{
        database::email::{CreateEmail, DeleteEmail, GetEmails, GetEmail},
        mailer::Mailer,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
};
use anyhow::Result;
use log::error;

#[post("")]
pub async fn send_email(
    exercise: ExerciseInfo,
    app_state: Data<AppState>,
    email_resource: Json<EmailResource>,
) -> Result<Json<Email>, RangerError> {
    let email;
    let email_resource = email_resource.into_inner();
    let mailer_configuration = app_state.configuration.mailer_configuration.clone();

    if let Some(mailer_configuration) = mailer_configuration {
        let mailer = Mailer::new(mailer_configuration.clone());

        let message = email_resource
            .create_message(mailer_configuration.from_address.clone())
            .map_err(|error| {
                error!("Failed to create message: {error}");
                RangerError::EmailMessageCreationFailed
            })?;

        mailer.send_message(message).map_err(|error| {
            error!("Failed to send email: {error}");
            RangerError::EmailSendingFailed
        })?;

        let new_email = NewEmail::new(
            email_resource,
            mailer_configuration.from_address,
            exercise.id,
            None,
        );

        email = app_state
            .database_address
            .send(CreateEmail(new_email.clone()))
            .await
            .map_err(create_mailbox_error_handler("Database"))?
            .map_err(create_database_error_handler("Create email"))?;
    } else {
        return Err(RangerError::MailerConfigurationNotFound);
    }

    Ok(Json(email))
}

#[get("")]
pub async fn get_emails(app_state: Data<AppState>) -> Result<Json<Vec<Email>>, RangerError> {
    let emails = app_state
        .database_address
        .send(GetEmails)
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get emails"))?;

    Ok(Json(emails))
}

#[get("{email_uuid}")]
pub async fn get_email(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Email>, RangerError> {
    let (_, email_id) = path_variables.into_inner();
    let email = app_state
        .database_address
        .send(GetEmail(email_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get email"))?;

    Ok(Json(email))
}

#[delete("{email_uuid}")]
pub async fn delete_email(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<String, RangerError> {
    let (_, email_id) = path_variables.into_inner();
    app_state
        .database_address
        .send(DeleteEmail(email_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Delete email"))?;

    Ok(email_id.to_string())
}

#[get("")]
pub async fn get_email_form(app_state: Data<AppState>) -> Result<Json<String>, RangerError> {
    let mailer_configuration = app_state.configuration.mailer_configuration.clone();
    let from_address;

    if let Some(mailer_configuration) = mailer_configuration {
        from_address = mailer_configuration.from_address;
    } else {
        return Err(RangerError::MailerConfigurationNotFound);
    }

    Ok(Json(from_address))
}
