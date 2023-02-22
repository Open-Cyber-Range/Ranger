use crate::{errors::RangerError, models::Email, services::mailer::Mailer, AppState};
use actix_web::{
    post,
    web::{Data, Json},
};
use anyhow::Result;
use log::{error, info};

#[post("exercise/{exercise_uuid}/email")]
pub async fn send_email(
    app_state: Data<AppState>,
    email: Json<Email>,
) -> Result<Json<Email>, RangerError> {
    let email = email.into_inner();
    let mailer_configuration = app_state.configuration.mailer_configuration.clone();

    if let Some(mailer_configuration) = mailer_configuration {
        let mailer = Mailer::new(mailer_configuration);

        let message = email.create_message().map_err(|error| {
            error!("Failed to create message: {error}");
            RangerError::EmailMessageCreationFailed
        })?;

        match mailer.send_message(message) {
            Ok(_) => info!("Mail sent successfully!"),
            Err(e) => error!("Mailer failed: {:?}", e),
        }
    } else {
        return Err(RangerError::MailerConfigurationNotFound);
    }

    Ok(Json(email))
}
