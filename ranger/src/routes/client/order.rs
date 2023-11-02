use crate::{
    errors::RangerError,
    middleware::authentication::UserInfo,
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
use log::error;

#[get("")]
pub async fn get_orders_client(
    app_state: Data<AppState>,
    user_info: UserInfo,
) -> Result<Json<Vec<Order>>, RangerError> {
    let client_id = user_info.email.clone().ok_or_else(|| {
        error!("Client id not found");
        RangerError::UserInfoMissing
    })?;

    let orders = app_state
        .database_address
        .send(GetOrders)
        .await
        .map_err(create_mailbox_error_handler("Database for orders"))?
        .map_err(create_database_error_handler("Get orders"))?
        .into_iter()
        .filter(|order| order.is_owner(&client_id))
        .collect();

    Ok(Json(orders))
}
