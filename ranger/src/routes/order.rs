use crate::{
    errors::RangerError,
    middleware::order::OrderInfo,
    models::{helpers::uuid::Uuid, NewOrder, Order, OrderRest, TrainingObjectiveRest},
    services::database::order::{
        CreateOrder, DeleteTrainingObjective, GetTrainingObjectivesByOrder, UpsertTrainingObjective,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
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

#[post("/training_objective")]
pub async fn create_training_objective(
    order: OrderInfo,
    app_state: Data<AppState>,
    new_training_objectives: Json<TrainingObjectiveRest>,
) -> Result<Json<TrainingObjectiveRest>, RangerError> {
    app_state
        .database_address
        .send(UpsertTrainingObjective(
            order.id,
            None,
            new_training_objectives.clone(),
        ))
        .await
        .map_err(create_mailbox_error_handler(
            "Database for training objectives",
        ))?
        .map_err(create_database_error_handler("Upsert training objective"))?;

    Ok(Json(new_training_objectives.into_inner()))
}

#[delete("/training_objective/{training_objective_uuid}")]
pub async fn delete_training_objective(
    _order: OrderInfo,
    path_variable: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Uuid>, RangerError> {
    let (_, training_objective_uuid) = path_variable.into_inner();
    app_state
        .database_address
        .send(DeleteTrainingObjective(training_objective_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database for orders"))?
        .map_err(create_database_error_handler("Delete training objective"))?;

    Ok(Json(training_objective_uuid))
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
    let training_objectives: Vec<TrainingObjectiveRest> = training_objectives
        .into_iter()
        .map(|threats_by_objective| threats_by_objective.into())
        .collect();

    Ok(Json(OrderRest::from((order, training_objectives))))
}
