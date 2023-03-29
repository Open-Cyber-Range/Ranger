use crate::{
    errors::RangerError,
    models::{Email, EmailResource},
    services::mailer::Mailer,
    AppState,
};
use actix_web::{
    get, post,
    web::{Data, Json},
};
use anyhow::Result;
use log::error;

#[post("exercise/{exercise_uuid}/email")]
pub async fn send_email(
    app_state: Data<AppState>,
    email_resource: Json<EmailResource>,
) -> Result<Json<Email>, RangerError> {
    let email;
    let email_resource = email_resource.into_inner();
    let mailer_configuration = app_state.configuration.mailer_configuration.clone();

    if let Some(mailer_configuration) = mailer_configuration {
        let mailer = Mailer::new(mailer_configuration.clone());
        email = Email::new(email_resource, mailer_configuration.from_address);

        let message = email.create_message().map_err(|error| {
            error!("Failed to create message: {error}");
            RangerError::EmailMessageCreationFailed
        })?;

        mailer.send_message(message).map_err(|error| {
            error!("Failed to send email: {error}");
            RangerError::EmailSendingFailed
        })?;
    } else {
        return Err(RangerError::MailerConfigurationNotFound);
    }

    Ok(Json(email))
}

#[get("exercise/{exercise_uuid}/email")]
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
