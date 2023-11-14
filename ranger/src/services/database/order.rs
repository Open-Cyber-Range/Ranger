use std::collections::HashMap;

use super::Database;
use crate::models::{
    helpers::uuid::Uuid, NewOrder, Order, Threat, TrainingObjective, TrainingObjectiveRest,
};
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

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct UpsertTrainingObjective(pub Uuid, pub Option<Uuid>, pub TrainingObjectiveRest);

impl Handler<UpsertTrainingObjective> for Database {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: UpsertTrainingObjective, _ctx: &mut Self::Context) -> Self::Result {
        let UpsertTrainingObjective(
            order_uuid,
            existing_training_objective_uuid,
            training_objective_rest,
        ) = msg;
        let training_objective =
            TrainingObjective::new(order_uuid, training_objective_rest.objective.clone());
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                block(move || {
                    if let Some(existing_training_objective_uuid) = existing_training_objective_uuid
                    {
                        TrainingObjective::hard_delete_by_id(existing_training_objective_uuid)
                            .execute(&mut connection)?;
                    }
                    training_objective
                        .create_insert()
                        .execute(&mut connection)?;
                    let threats = training_objective_rest
                        .threats
                        .into_iter()
                        .map(|threat| Threat::new(training_objective.id, threat))
                        .collect::<Vec<Threat>>();
                    Threat::batch_insert(threats).execute(&mut connection)?;

                    Ok(())
                })
                .await??;

                Ok(())
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<HashMap<TrainingObjective, Vec<Threat>>>")]
pub struct GetTrainingObjectivesByOrder(pub Order);

impl Handler<GetTrainingObjectivesByOrder> for Database {
    type Result = ResponseActFuture<Self, Result<HashMap<TrainingObjective, Vec<Threat>>>>;

    fn handle(
        &mut self,
        get_objectives: GetTrainingObjectivesByOrder,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let connection_result = self.get_connection();
        let GetTrainingObjectivesByOrder(order) = get_objectives;

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let objectives = block(move || {
                    let training_objectives =
                        TrainingObjective::by_order(&order).load(&mut connection)?;

                    let mut threats_by_objectives: HashMap<TrainingObjective, Vec<Threat>> =
                        HashMap::new();
                    for trainining_objective in &training_objectives {
                        let threats =
                            Threat::by_objective(trainining_objective).load(&mut connection)?;
                        threats_by_objectives.insert(trainining_objective.clone(), threats);
                    }

                    Ok(threats_by_objectives)
                })
                .await??;

                Ok(objectives)
            }
            .into_actor(self),
        )
    }
}
