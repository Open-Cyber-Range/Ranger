use crate::{
    errors::RangerError,
    middleware::{authentication::UserInfo, order::OrderInfo},
    models::{helpers::uuid::Uuid, Order, StructureRest, TrainingObjectiveRest},
    services::database::order::{
        DeleteStructure, DeleteTrainingObjective, GetOrders, UpsertStructure,
        UpsertTrainingObjective,
    },
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
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

#[put("/training_objective/{training_objective_uuid}")]
pub async fn update_training_objective(
    order: OrderInfo,
    path_variable: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
    new_training_objective: Json<TrainingObjectiveRest>,
) -> Result<Json<TrainingObjectiveRest>, RangerError> {
    let (_, training_objective_uuid) = path_variable.into_inner();
    app_state
        .database_address
        .send(UpsertTrainingObjective(
            order.id,
            Some(training_objective_uuid),
            new_training_objective.clone(),
        ))
        .await
        .map_err(create_mailbox_error_handler(
            "Database for training objectives",
        ))?
        .map_err(create_database_error_handler("Upsert training objective"))?;

    Ok(Json(new_training_objective.into_inner()))
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
        .map_err(create_mailbox_error_handler(
            "Database for training objectives",
        ))?
        .map_err(create_database_error_handler("Delete training objective"))?;

    Ok(Json(training_objective_uuid))
}

#[post("/structure")]
pub async fn create_structure(
    order: OrderInfo,
    app_state: Data<AppState>,
    new_structure: Json<StructureRest>,
) -> Result<Json<StructureRest>, RangerError> {
    app_state
        .database_address
        .send(UpsertStructure(order.id, None, new_structure.clone()))
        .await
        .map_err(create_mailbox_error_handler(
            "Database for client order structures",
        ))?
        .map_err(create_database_error_handler("Create structure"))?;

    Ok(Json(new_structure.into_inner()))
}

#[delete("/structure/{structure_uuid}")]
pub async fn delete_structure(
    _order: OrderInfo,
    path_variable: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Json<Uuid>, RangerError> {
    let (_, structure_uuid) = path_variable.into_inner();
    app_state
        .database_address
        .send(DeleteStructure(structure_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database for structures"))?
        .map_err(create_database_error_handler("Delete structure"))?;

    Ok(Json(structure_uuid))
}

#[put("/structure/{structure_uuid}")]
pub async fn update_structure(
    order: OrderInfo,
    path_variable: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
    new_structure: Json<StructureRest>,
) -> Result<Json<StructureRest>, RangerError> {
    let (_, structure_uuid) = path_variable.into_inner();
    app_state
        .database_address
        .send(UpsertStructure(
            order.id,
            Some(structure_uuid),
            new_structure.clone(),
        ))
        .await
        .map_err(create_mailbox_error_handler("Database for structure"))?
        .map_err(create_database_error_handler("Upsert structure"))?;

    Ok(Json(new_structure.into_inner()))
}
