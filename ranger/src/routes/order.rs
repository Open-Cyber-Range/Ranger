use crate::{
    errors::RangerError,
    middleware::order::OrderInfo,
    models::{NewOrder, Order},
    services::database::order::CreateOrder,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    get, post,
    web::{Data, Json},
};
use anyhow::Result;

#[post("")]
pub async fn create_order(
    app_state: Data<AppState>,
    new_order: Json<NewOrder>,
) -> Result<Json<Order>, RangerError> {
    let order = app_state
        .database_address
        .send(CreateOrder(new_order.into_inner()))
        .await
        .map_err(create_mailbox_error_handler("Database for orders"))?
        .map_err(create_database_error_handler("Create order"))?;

    Ok(Json(order))
}

#[get("")]
pub async fn get_order(order: OrderInfo) -> Result<Json<Order>, RangerError> {
    let order = order.into_inner();

    Ok(Json(order))
}
