use super::Database;
use crate::models::{helpers::uuid::Uuid, Email, NewEmail};
use actix::{Handler, Message, ResponseActFuture, WrapFuture};
use actix_web::web::block;
use anyhow::{Ok, Result};
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<Email>")]
pub struct CreateEmail(pub NewEmail);

impl Handler<CreateEmail> for Database {
    type Result = ResponseActFuture<Self, Result<Email>>;

    fn handle(&mut self, msg: CreateEmail, _ctx: &mut Self::Context) -> Self::Result {
        let new_email = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let email = block(move || {
                    new_email.create_insert().execute(&mut connection)?;
                    let email = Email::by_id(new_email.id).first(&mut connection)?;

                    Ok(email)
                })
                .await??;

                Ok(email)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Email>")]
pub struct GetEmail(pub Uuid);

impl Handler<GetEmail> for Database {
    type Result = ResponseActFuture<Self, Result<Email>>;

    fn handle(&mut self, msg: GetEmail, _ctx: &mut Self::Context) -> Self::Result {
        let email_id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let email = block(move || {
                    let email = Email::by_id(email_id).first(&mut connection)?;

                    Ok(email)
                })
                .await??;

                Ok(email)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Email>>")]
pub struct GetEmails(pub Uuid);

impl Handler<GetEmails> for Database {
    type Result = ResponseActFuture<Self, Result<Vec<Email>>>;

    fn handle(&mut self, msg: GetEmails, _ctx: &mut Self::Context) -> Self::Result {
        let exercise_id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let emails = block(move || {
                    let emails = Email::by_exercise_id(exercise_id).load(&mut connection)?;

                    Ok(emails)
                })
                .await??;

                Ok(emails)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid>")]
pub struct DeleteEmail(pub Uuid);

impl Handler<DeleteEmail> for Database {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: DeleteEmail, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let id = block(move || {
                    let email = Email::by_id(id).first(&mut connection)?;
                    email.hard_delete().execute(&mut connection)?;
                    Ok(id)
                })
                .await??;

                Ok(id)
            }
            .into_actor(self),
        )
    }
}
