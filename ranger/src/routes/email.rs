use crate::{
    errors::RangerError,
    middleware::exercise::ExerciseInfo,
    models::{
        helpers::uuid::Uuid, Email, EmailResource, EmailWithStatus, NewEmail,
        NewEmailStatus,
    },
    services::{
        database::{
            email::{CreateEmail, DeleteEmail, GetEmail, GetEmails},
            email_status::{CreateEmailStatus, GetEmailStatus},
        },
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
        let new_email = NewEmail::new(
            email_resource.clone(),
            mailer_configuration.from_address.clone(),
            exercise.id,
        );
        let email_status_pending = NewEmailStatus::new_pending(new_email.id);

        email = app_state
            .database_address
            .send(CreateEmail(new_email.clone()))
            .await
            .map_err(create_mailbox_error_handler("Database"))?
            .map_err(create_database_error_handler("Create email"))?;

        app_state
            .database_address
            .send(CreateEmailStatus(email_status_pending))
            .await
            .map_err(create_mailbox_error_handler("Database"))?
            .map_err(create_database_error_handler("Create email status"))?;

        let message = match email_resource.create_message(mailer_configuration.from_address) {
            Ok(message) => message,
            Err(error) => {
                error!("Failed to create message: {error}");
                let email_status_error =
                    NewEmailStatus::new_failed(new_email.id, error.to_string());

                app_state
                    .database_address
                    .send(CreateEmailStatus(email_status_error))
                    .await
                    .map_err(create_mailbox_error_handler("Database"))?
                    .map_err(create_database_error_handler("Create email status"))?;

                return Err(RangerError::EmailMessageCreationFailed);
            }
        };

        match mailer.send_message(message) {
            Ok(response) => {
                let email_status_sent =
                    NewEmailStatus::new_sent(new_email.id, response.first_line());
                app_state
                    .database_address
                    .send(CreateEmailStatus(email_status_sent))
                    .await
                    .map_err(create_mailbox_error_handler("Database"))?
                    .map_err(create_database_error_handler("Create email status"))?;
            }
            Err(error) => {
                error!("Failed to send email: {error}");
                let email_status_error =
                    NewEmailStatus::new_failed(new_email.id, error.to_string());

                app_state
                    .database_address
                    .send(CreateEmailStatus(email_status_error))
                    .await
                    .map_err(create_mailbox_error_handler("Database"))?
                    .map_err(create_database_error_handler("Create email status"))?;
            }
        }
    } else {
        return Err(RangerError::MailerConfigurationNotFound);
    }

    Ok(Json(email))
}

#[get("")]
pub async fn get_emails(
    path_variables: Path<Uuid>,
    app_state: Data<AppState>,
) -> Result<Json<Vec<EmailWithStatus>>, RangerError> {
    let exercise_id = path_variables.into_inner();
    let emails = app_state
        .database_address
        .send(GetEmails(exercise_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get emails"))?;

    let email_status_futures = emails.into_iter().map(|email| {
        let app_state_clone = app_state.clone();
        async move {
            let email_status = app_state_clone
                .database_address
                .send(GetEmailStatus(email.id))
                .await
                .map_err(create_mailbox_error_handler("Database"))?
                .map_err(create_database_error_handler("Get email status"))?;

            Ok(EmailWithStatus::new(email, email_status))
        }
    });

    let email_status_results: Result<Vec<EmailWithStatus>, RangerError> =
        futures::future::try_join_all(email_status_futures).await;

    let emails_with_statuses = email_status_results?;

    Ok(Json(emails_with_statuses))
}

#[get("{email_uuid}")]
pub async fn get_email(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<EmailWithStatus>, RangerError> {
    let (_, email_id) = path_variables.into_inner();
    let email = app_state
        .database_address
        .send(GetEmail(email_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get email"))?;

    let email_status = app_state
        .database_address
        .send(GetEmailStatus(email_id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get email status"))?;

    let email_with_status = EmailWithStatus::new(email, email_status);

    Ok(Json(email_with_status))
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
