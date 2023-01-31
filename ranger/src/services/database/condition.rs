use super::Database;
use crate::models::helpers::uuid::Uuid;
use crate::models::{ConditionMessage, NewConditionMessage};
use actix::{Handler, Message, ResponseActFuture, WrapFuture};
use actix_web::web::block;
use anyhow::{Ok, Result};
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<ConditionMessage>")]
pub struct CreateConditionMessage(pub NewConditionMessage);

impl Handler<CreateConditionMessage> for Database {
    type Result = ResponseActFuture<Self, Result<ConditionMessage>>;

    fn handle(&mut self, msg: CreateConditionMessage, _ctx: &mut Self::Context) -> Self::Result {
        let new_condition_message = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let condition_message = block(move || {
                    new_condition_message
                        .create_insert()
                        .execute(&mut connection)?;
                    let condition_message =
                        ConditionMessage::by_id(new_condition_message.id).first(&mut connection)?;

                    Ok(condition_message)
                })
                .await??;

                Ok(condition_message)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<ConditionMessage>")]
pub struct GetConditionMessage(pub Uuid);

impl Handler<GetConditionMessage> for Database {
    type Result = ResponseActFuture<Self, Result<ConditionMessage>>;

    fn handle(&mut self, msg: GetConditionMessage, _ctx: &mut Self::Context) -> Self::Result {
        let connection_result = self.get_connection();
        let id = msg.0;

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let condition_message = block(move || {
                    let condition_message = ConditionMessage::by_id(id).first(&mut connection)?;

                    Ok(condition_message)
                })
                .await??;

                Ok(condition_message)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid>")]
pub struct DeleteConditionMessage(pub Uuid);

impl Handler<DeleteConditionMessage> for Database {
    type Result = ResponseActFuture<Self, Result<Uuid>>;

    fn handle(&mut self, msg: DeleteConditionMessage, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let id = block(move || {
                    let condition_message = ConditionMessage::by_id(id).first(&mut connection)?;
                    condition_message.soft_delete().execute(&mut connection)?;

                    Ok(id)
                })
                .await??;

                Ok(id)
            }
            .into_actor(self),
        )
    }
}
