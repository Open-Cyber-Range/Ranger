use super::Database;
use crate::models::{helpers::uuid::Uuid, NewOrder, Order};
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

#[derive(Message)]
#[rtype(result = "Result<Order>")]
pub struct GetOrder(pub Uuid);

impl Handler<GetOrder> for Database {
    type Result = ResponseActFuture<Self, Result<Order>>;

    fn handle(&mut self, msg: GetOrder, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = msg.0;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let order = block(move || {
                    let order = Order::by_id(uuid).first(&mut connection)?;

                    Ok(order)
                })
                .await??;

                Ok(order)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Order>>")]
pub struct GetOrders;

impl Handler<GetOrders> for Database {
    type Result = ResponseActFuture<Self, Result<Vec<Order>>>;

    fn handle(&mut self, _: GetOrders, _ctx: &mut Self::Context) -> Self::Result {
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let order = block(move || {
                    let orders = Order::all().load(&mut connection)?;

                    Ok(orders)
                })
                .await??;

                Ok(order)
            }
            .into_actor(self),
        )
    }
}
