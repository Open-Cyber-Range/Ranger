use crate::models::helpers::uuid::Uuid;
use crate::models::{EmailTemplate, NewEmailTemplate};
use crate::services::database::email::{
    CreateEmailTemplate, DeleteEmailTemplate, GetEmailTemplate, GetEmailTemplates,
};
use crate::utilities::{create_database_error_handler, create_mailbox_error_handler};
use crate::{
    errors::RangerError,
    models::{Email, EmailResource},
    services::mailer::Mailer,
    AppState,
};
use actix_web::web::Path;
use actix_web::{
    delete, get, post,
    web::{Data, Json},
};
use anyhow::Result;
use log::error;

#[post("email")]
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

#[get("email")]
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

#[post("")]
pub async fn add_emailtemplate(
    app_state: Data<AppState>,
    new_emailtemplate_json: Json<NewEmailTemplate>,
) -> Result<Json<EmailTemplate>, RangerError> {
    let new_emailtemplate = new_emailtemplate_json.into_inner();
    let emailtemplate = app_state
        .database_address
        .send(CreateEmailTemplate(new_emailtemplate))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Create emailtemplate"))?;
    log::debug!("Created emailtemplate: {}", emailtemplate.id);
    Ok(Json(emailtemplate))
}

#[get("")]
pub async fn get_emailtemplates(
    app_state: Data<AppState>,
) -> Result<Json<Vec<EmailTemplate>>, RangerError> {
    let emailtemplates = app_state
        .database_address
        .send(GetEmailTemplates)
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get emailtemplates"))?;
    Ok(Json(emailtemplates))
}

#[get("")]
pub async fn get_emailtemplate(
    path_variable: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<EmailTemplate>, RangerError> {
    let (_, id) = path_variable.into_inner();
    let emailtemplate = app_state
        .database_address
        .send(GetEmailTemplate(id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get emailtemplate"))?;
    Ok(Json(emailtemplate))
}

#[delete("")]
pub async fn delete_emailtemplate(
    path_variable: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<String, RangerError> {
    let (_, id) = path_variable.into_inner();
    app_state
        .database_address
        .send(DeleteEmailTemplate(id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Delete emailtemplate"))?;
    log::debug!("Deleted emailtemplate {}", id);
    Ok(id.to_string())
}
