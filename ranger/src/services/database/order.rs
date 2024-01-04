use std::collections::HashMap;

use super::Database;
use crate::models::{
    helpers::uuid::Uuid, NewOrder, Order, Skill, SkillRest, Structure, StructureObjective,
    StructureObjectiveRest, StructureRest, StructureWithElements, Threat, ThreatRest,
    TrainingObjective, TrainingObjectiveRest, Weakness, WeaknessRest,
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
                        .map(|threat| Threat::new(training_objective.id, threat.threat))
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
#[rtype(result = "Result<HashMap<TrainingObjective, Vec<ThreatRest>>>")]
pub struct GetTrainingObjectivesByOrder(pub Order);

impl Handler<GetTrainingObjectivesByOrder> for Database {
    type Result = ResponseActFuture<Self, Result<HashMap<TrainingObjective, Vec<ThreatRest>>>>;

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

                    let mut threats_by_objectives: HashMap<TrainingObjective, Vec<ThreatRest>> =
                        HashMap::new();
                    for trainining_objective in &training_objectives {
                        let threats = Threat::by_objective(trainining_objective)
                            .load(&mut connection)?
                            .into_iter()
                            .map(|threat| threat.into())
                            .collect::<Vec<ThreatRest>>();
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

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct DeleteTrainingObjective(pub Uuid);

impl Handler<DeleteTrainingObjective> for Database {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: DeleteTrainingObjective, _ctx: &mut Self::Context) -> Self::Result {
        let DeleteTrainingObjective(training_objective_uuid) = msg;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                block(move || {
                    let training_objective =
                        TrainingObjective::by_id(training_objective_uuid).first(&mut connection)?;
                    training_objective.hard_delete().execute(&mut connection)?;

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
#[rtype(result = "Result<()>")]
pub struct UpsertStructure(pub Uuid, pub Option<Uuid>, pub StructureRest);

impl Handler<UpsertStructure> for Database {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: UpsertStructure, _ctx: &mut Self::Context) -> Self::Result {
        let UpsertStructure(order_uuid, structure_uuid, new_structure) = msg;
        let new_skills = new_structure.skills.clone();
        let weaknesses = new_structure.weaknesses.clone();
        let training_objectives = new_structure.training_objective_ids.clone();
        let structure = Structure::new(order_uuid, new_structure);
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                block(move || {
                    if let Some(structure_uuid) = structure_uuid {
                        Structure::hard_delete_by_id(structure_uuid).execute(&mut connection)?;
                    }
                    structure.create_insert().execute(&mut connection)?;
                    if let Some(skills) = new_skills {
                        let skills = skills
                            .into_iter()
                            .map(|skill| Skill::new(structure.id, skill))
                            .collect::<Vec<Skill>>();
                        Skill::batch_insert(skills).execute(&mut connection)?;
                    }
                    if let Some(weaknesses) = weaknesses {
                        let weaknesses = weaknesses
                            .into_iter()
                            .map(|weakness| Weakness::new(structure.id, weakness))
                            .collect::<Vec<Weakness>>();
                        Weakness::batch_insert(weaknesses).execute(&mut connection)?;
                    }
                    if let Some(training_objectives) = training_objectives {
                        let training_objectives = training_objectives
                            .into_iter()
                            .map(|training_objective| {
                                StructureObjective::new(structure.id, training_objective)
                            })
                            .collect::<Vec<StructureObjective>>();
                        StructureObjective::batch_insert(training_objectives)
                            .execute(&mut connection)?;
                    }

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
#[rtype(result = "Result<StructureWithElements>")]
pub struct GetStructuresByOrder(pub Order);

impl Handler<GetStructuresByOrder> for Database {
    type Result = ResponseActFuture<Self, Result<StructureWithElements>>;

    fn handle(&mut self, msg: GetStructuresByOrder, _ctx: &mut Self::Context) -> Self::Result {
        let connection_result = self.get_connection();
        let GetStructuresByOrder(order) = msg;

        Box::pin(
            async move {
                let mut connection = connection_result?;
                let objectives = block(move || {
                    let structures = Structure::by_order(&order).load(&mut connection)?;

                    let mut elements_by_structure: StructureWithElements = HashMap::new();
                    for structure in &structures {
                        let skills = Skill::by_structure(structure)
                            .load(&mut connection)?
                            .into_iter()
                            .map(|skill| skill.into())
                            .collect::<Vec<SkillRest>>();
                        let training_objectives = StructureObjective::by_structure(structure)
                            .load(&mut connection)?
                            .into_iter()
                            .map(|structure_objective| structure_objective.into())
                            .collect::<Vec<StructureObjectiveRest>>();
                        let weaknesses = Weakness::by_structure(structure)
                            .load(&mut connection)?
                            .into_iter()
                            .map(|weakness| weakness.into())
                            .collect::<Vec<WeaknessRest>>();
                        elements_by_structure
                            .insert(structure.clone(), (skills, training_objectives, weaknesses));
                    }

                    Ok(elements_by_structure)
                })
                .await??;

                Ok(objectives)
            }
            .into_actor(self),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct DeleteStructure(pub Uuid);

impl Handler<DeleteStructure> for Database {
    type Result = ResponseActFuture<Self, Result<()>>;

    fn handle(&mut self, msg: DeleteStructure, _ctx: &mut Self::Context) -> Self::Result {
        let DeleteStructure(structure_uuid) = msg;
        let connection_result = self.get_connection();

        Box::pin(
            async move {
                let mut connection = connection_result?;
                block(move || {
                    let structure = Structure::by_id(structure_uuid).first(&mut connection)?;
                    structure.hard_delete().execute(&mut connection)?;

                    Ok(())
                })
                .await??;

                Ok(())
            }
            .into_actor(self),
        )
    }
}
