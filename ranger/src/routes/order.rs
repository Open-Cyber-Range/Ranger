use crate::{
    errors::RangerError,
    models::{NewOrder, Order},
    services::database::order::CreateOrder,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    post,
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

// #[get("order")]
// pub async fn get_order(app_state: Data<AppState>) -> Result<Json<String>, RangerError> {
//     let mailer_configuration = app_state.configuration.mailer_configuration.clone();
//     let from_address;

//     if let Some(mailer_configuration) = mailer_configuration {
//         from_address = mailer_configuration.from_address;
//     } else {
//         return Err(RangerError::MailerConfigurationNotFound);
//     }

//     Ok(Json(from_address))
// }
