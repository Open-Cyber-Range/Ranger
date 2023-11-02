use crate::{
    errors::RangerError,
    models::Order,
    services::database::order::GetOrders,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json},
};
use anyhow::Result;

#[get("")]
pub async fn get_orders_admin(app_state: Data<AppState>) -> Result<Json<Vec<Order>>, RangerError> {
    let orders = app_state
        .database_address
        .send(GetOrders)
        .await
        .map_err(create_mailbox_error_handler("Database for orders"))?
        .map_err(create_database_error_handler("Get orders"))?;

    Ok(Json(orders))
}
