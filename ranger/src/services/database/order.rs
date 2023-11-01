use super::Database;
use crate::models::{NewOrder, Order};
use actix::{Handler, Message, ResponseActFuture, WrapFuture};
use actix_web::web::block;
use anyhow::{Ok, Result};
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<Order>")]
pub struct CreateOrder(pub NewOrder);

impl Handler<CreateOrder> for Database {
    type Result = ResponseActFuture<Self, Result<Order>>;

    fn handle(&mut self, msg: CreateOrder, _ctx: &mut Self::Context) -> Self::Result {
        let new_order = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let order = block(move || {
                    new_order.create_insert().execute(&mut connection)?;
                    let order = Order::by_id(new_order.id).first(&mut connection)?;

                    Ok(order)
                })
                .await??;

                Ok(order)
            }
            .into_actor(self),
        )
    }
}
