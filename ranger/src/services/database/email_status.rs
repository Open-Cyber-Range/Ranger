use super::Database;
use crate::models::{helpers::uuid::Uuid, EmailStatus, NewEmailStatus};
use actix::{Handler, Message, ResponseActFuture, WrapFuture};
use actix_web::web::block;
use anyhow::{Ok, Result};
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<EmailStatus>")]
pub struct CreateEmailStatus(pub NewEmailStatus);

impl Handler<CreateEmailStatus> for Database {
    type Result = ResponseActFuture<Self, Result<EmailStatus>>;

    fn handle(&mut self, msg: CreateEmailStatus, _ctx: &mut Self::Context) -> Self::Result {
        let new_email_status = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let email_status = block(move || {
                    new_email_status.create_insert().execute(&mut connection)?;
                    let email_status =
                        EmailStatus::by_id(new_email_status.id).first(&mut connection)?;

                    Ok(email_status)
                })
                .await??;

                Ok(email_status)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<EmailStatus>")]
pub struct GetEmailStatus(pub Uuid);

impl Handler<GetEmailStatus> for Database {
    type Result = ResponseActFuture<Self, Result<EmailStatus>>;

    fn handle(&mut self, msg: GetEmailStatus, _ctx: &mut Self::Context) -> Self::Result {
        let connection_result = self.get_connection();
        let email_id = msg.0;

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let email_status = block(move || {
                    let email_status =
                        EmailStatus::by_email_id_latest(email_id).first(&mut connection)?;

                    Ok(email_status)
                })
                .await??;

                Ok(email_status)
            }
            .into_actor(self),
        )
    }
}
