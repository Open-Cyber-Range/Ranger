use crate::{
    errors::RangerError,
    middleware::order::OrderInfo,
    models::{NewOrder, Order, OrderRest, StructureWithElements},
    services::database::order::{
        CreateOrder, GetCustomElementsByOrder, GetEnvironmentsByOrder, GetPlotsByOrder,
        GetStructuresByOrder, GetTrainingObjectivesByOrder,
    },
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
pub async fn get_order(
    order: OrderInfo,
    app_state: Data<AppState>,
) -> Result<Json<OrderRest>, RangerError> {
    let order = order.into_inner();

    let training_objectives = app_state
        .database_address
        .send(GetTrainingObjectivesByOrder(order.clone()))
        .await
        .map_err(create_mailbox_error_handler(
            "Database for training objectives",
        ))?
        .map_err(create_database_error_handler("Get training objectives"))?;
    let structures: StructureWithElements = app_state
        .database_address
        .send(GetStructuresByOrder(order.clone()))
        .await
        .map_err(create_mailbox_error_handler("Database for structures"))?
        .map_err(create_database_error_handler("Get structures"))?;
    let environments = app_state
        .database_address
        .send(GetEnvironmentsByOrder(order.clone()))
        .await
        .map_err(create_mailbox_error_handler("Database for environments"))?
        .map_err(create_database_error_handler("Get environments"))?;
    let custom_elements = app_state
        .database_address
        .send(GetCustomElementsByOrder(order.clone()))
        .await
        .map_err(create_mailbox_error_handler("Database for custom elements"))?
        .map_err(create_database_error_handler("Get custom elements"))?;
    let plots = app_state
        .database_address
        .send(GetPlotsByOrder(order.clone()))
        .await
        .map_err(create_mailbox_error_handler("Database for plots"))?
        .map_err(create_database_error_handler("Get plots"))?;

    Ok(Json(OrderRest::from((
        order,
        training_objectives,
        structures,
        environments,
        custom_elements,
        plots,
    ))))
}
