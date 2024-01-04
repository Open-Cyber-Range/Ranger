use super::helpers::uuid::Uuid;
use crate::{
    constants::{MAX_ORDER_NAME_LENGTH, NAIVEDATETIME_DEFAULT_VALUE},
    errors::RangerError,
    schema::{
        orders, skills, structure_training_objectives, structure_weaknesses, structures, threats,
        training_objectives,
    },
    services::database::{
        All, Create, DeleteById, FilterExisting, SelectById, SelectByIdFromAll,
        SelectByIdFromAllReference,
    },
    utilities::Validation,
};
use chrono::NaiveDateTime;
use diesel::{
    associations::{Associations, Identifiable},
    insert_into,
    prelude::{Insertable, Queryable},
    BelongingToDsl, ExpressionMethods, QueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, result::Result as StdResult};

#[derive(Insertable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = orders)]
pub struct NewOrder {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub client_id: String,
}

impl NewOrder {
    pub fn create_insert(&self) -> Create<&Self, orders::table> {
        insert_into(orders::table).values(self)
    }
}

impl Validation for NewOrder {
    fn validate(&self) -> StdResult<(), RangerError> {
        if self.name.len() > MAX_ORDER_NAME_LENGTH {
            return Err(RangerError::OrderNameTooLong);
        }
        Ok(())
    }
}

#[derive(
    Identifiable, Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = orders)]
pub struct Order {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub client_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Order {
    fn all_with_deleted() -> All<orders::table, Self> {
        orders::table.select(Self::as_select())
    }

    pub fn all() -> FilterExisting<All<orders::table, Self>, orders::deleted_at> {
        Self::all_with_deleted().filter(orders::deleted_at.eq(*NAIVEDATETIME_DEFAULT_VALUE))
    }

    pub fn by_id(id: Uuid) -> SelectById<orders::table, orders::id, orders::deleted_at, Self> {
        Self::all().filter(orders::id.eq(id))
    }

    pub fn is_owner(&self, client_id: &str) -> bool {
        self.client_id == client_id
    }
}

#[derive(
    Insertable, Identifiable, Queryable, Selectable, Debug, PartialEq, Associations, Eq, Clone, Hash,
)]
#[diesel(belongs_to(Order, foreign_key = order_id))]
#[diesel(table_name = training_objectives)]
pub struct TrainingObjective {
    pub id: Uuid,
    pub order_id: Uuid,
    pub objective: String,
}

impl TrainingObjective {
    pub fn new(order_id: Uuid, objective: String) -> Self {
        Self {
            id: Uuid::random(),
            order_id,
            objective,
        }
    }

    pub fn create_insert(&self) -> Create<&Self, training_objectives::table> {
        insert_into(training_objectives::table).values(self)
    }

    pub fn hard_delete_by_id(
        id: Uuid,
    ) -> DeleteById<training_objectives::id, training_objectives::table> {
        diesel::delete(training_objectives::table.filter(training_objectives::id.eq(id)))
    }

    pub fn hard_delete(&self) -> DeleteById<training_objectives::id, training_objectives::table> {
        Self::hard_delete_by_id(self.id)
    }

    fn all() -> All<training_objectives::table, Self> {
        training_objectives::table.select(Self::as_select())
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectByIdFromAll<training_objectives::table, training_objectives::id, Self> {
        Self::all().filter(training_objectives::id.eq(id))
    }

    pub fn by_order(
        order: &Order,
    ) -> SelectByIdFromAllReference<training_objectives::table, training_objectives::order_id, Self>
    {
        Self::belonging_to(order).select(Self::as_select())
    }
}

#[derive(
    Insertable,
    Identifiable,
    Associations,
    Queryable,
    Selectable,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(TrainingObjective, foreign_key = training_objective_id))]
#[diesel(table_name = threats)]
pub struct Threat {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub training_objective_id: Uuid,
    pub threat: String,
}

impl Threat {
    pub fn new(training_objective_id: Uuid, threat: String) -> Self {
        Self {
            id: Uuid::random(),
            training_objective_id,
            threat,
        }
    }

    pub fn by_objective(
        objective: &TrainingObjective,
    ) -> SelectByIdFromAllReference<threats::table, threats::training_objective_id, Self> {
        Self::belonging_to(objective).select(Self::as_select())
    }

    pub fn batch_insert(threats: Vec<Self>) -> Create<Vec<Self>, threats::table> {
        insert_into(threats::table).values(threats)
    }
}

#[derive(
    Insertable,
    Identifiable,
    Queryable,
    Selectable,
    Debug,
    PartialEq,
    Associations,
    Eq,
    Clone,
    Hash,
    Serialize,
    Deserialize,
)]
#[diesel(belongs_to(Order, foreign_key = order_id))]
#[diesel(table_name = structures)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
    pub id: Uuid,
    pub order_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
}

impl Structure {
    pub fn new(order_id: Uuid, new_structure: StructureRest) -> Self {
        Self {
            id: new_structure.id,
            order_id,
            name: new_structure.name,
            description: new_structure.description,
            parent_id: new_structure.parent_id,
        }
    }

    pub fn create_insert(&self) -> Create<&Self, structures::table> {
        insert_into(structures::table).values(self)
    }

    pub fn hard_delete_by_id(id: Uuid) -> DeleteById<structures::id, structures::table> {
        diesel::delete(structures::table.filter(structures::id.eq(id)))
    }

    pub fn hard_delete(&self) -> DeleteById<structures::id, structures::table> {
        Self::hard_delete_by_id(self.id)
    }

    fn all() -> All<structures::table, Self> {
        structures::table.select(Self::as_select())
    }

    pub fn by_id(id: Uuid) -> SelectByIdFromAll<structures::table, structures::id, Self> {
        Self::all().filter(structures::id.eq(id))
    }

    pub fn by_order(
        order: &Order,
    ) -> SelectByIdFromAllReference<structures::table, structures::order_id, Self> {
        Self::belonging_to(order).select(Self::as_select())
    }
}

#[derive(
    Insertable,
    Identifiable,
    Associations,
    Queryable,
    Selectable,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Structure, foreign_key = structure_id))]
#[diesel(table_name = skills)]
pub struct Skill {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub structure_id: Uuid,
    pub skill: String,
}

impl Skill {
    pub fn new(structure_id: Uuid, skill: SkillRest) -> Self {
        Self {
            id: skill.id,
            structure_id,
            skill: skill.skill,
        }
    }

    pub fn by_structure(
        structure: &Structure,
    ) -> SelectByIdFromAllReference<skills::table, skills::structure_id, Self> {
        Self::belonging_to(structure).select(Self::as_select())
    }

    pub fn batch_insert(skills: Vec<Self>) -> Create<Vec<Self>, skills::table> {
        insert_into(skills::table).values(skills)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRest {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub skill: String,
}

impl From<Skill> for SkillRest {
    fn from(skill: Skill) -> Self {
        Self {
            id: skill.id,
            skill: skill.skill,
        }
    }
}

#[derive(
    Insertable,
    Identifiable,
    Associations,
    Queryable,
    Selectable,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Structure, foreign_key = structure_id))]
#[diesel(table_name = structure_training_objectives)]
pub struct StructureObjective {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub structure_id: Uuid,
    pub training_objective_id: Uuid,
}

impl StructureObjective {
    pub fn new(structure_id: Uuid, structure_objective: StructureObjectiveRest) -> Self {
        Self {
            id: structure_objective.id,
            structure_id,
            training_objective_id: structure_objective.training_objective_id,
        }
    }

    pub fn by_structure(
        structure: &Structure,
    ) -> SelectByIdFromAllReference<
        structure_training_objectives::table,
        structure_training_objectives::structure_id,
        Self,
    > {
        Self::belonging_to(structure).select(Self::as_select())
    }

    pub fn batch_insert(
        training_objective_ids: Vec<Self>,
    ) -> Create<Vec<Self>, structure_training_objectives::table> {
        insert_into(structure_training_objectives::table).values(training_objective_ids)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureObjectiveRest {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub training_objective_id: Uuid,
}

impl From<StructureObjective> for StructureObjectiveRest {
    fn from(structure_objective: StructureObjective) -> Self {
        Self {
            id: structure_objective.id,
            training_objective_id: structure_objective.training_objective_id,
        }
    }
}

#[derive(
    Insertable,
    Identifiable,
    Associations,
    Queryable,
    Selectable,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Structure, foreign_key = structure_id))]
#[diesel(table_name = structure_weaknesses)]
pub struct Weakness {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub structure_id: Uuid,
    pub weakness: String,
}

impl Weakness {
    pub fn new(structure_id: Uuid, weakness: WeaknessRest) -> Self {
        Self {
            id: weakness.id,
            structure_id,
            weakness: weakness.weakness,
        }
    }

    pub fn by_structure(
        structure: &Structure,
    ) -> SelectByIdFromAllReference<
        structure_weaknesses::table,
        structure_weaknesses::structure_id,
        Self,
    > {
        Self::belonging_to(structure).select(Self::as_select())
    }

    pub fn batch_insert(weaknesses: Vec<Self>) -> Create<Vec<Self>, structure_weaknesses::table> {
        insert_into(structure_weaknesses::table).values(weaknesses)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaknessRest {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub weakness: String,
}

impl From<Weakness> for WeaknessRest {
    fn from(weakness: Weakness) -> Self {
        Self {
            id: weakness.id,
            weakness: weakness.weakness,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureRest {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub weaknesses: Option<Vec<WeaknessRest>>,
    pub skills: Option<Vec<SkillRest>>,
    pub training_objective_ids: Option<Vec<StructureObjectiveRest>>,
}

impl
    From<(
        Structure,
        (
            Vec<SkillRest>,
            Vec<StructureObjectiveRest>,
            Vec<WeaknessRest>,
        ),
    )> for StructureRest
{
    fn from(
        (structure, (skills, structure_objectives, weaknesses)): (
            Structure,
            (
                Vec<SkillRest>,
                Vec<StructureObjectiveRest>,
                Vec<WeaknessRest>,
            ),
        ),
    ) -> Self {
        Self {
            id: structure.id,
            name: structure.name,
            description: structure.description,
            parent_id: structure.parent_id,
            weaknesses: Some(weaknesses),
            skills: Some(skills),
            training_objective_ids: Some(structure_objectives),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatRest {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub threat: String,
}

impl From<Threat> for ThreatRest {
    fn from(threat: Threat) -> Self {
        Self {
            id: threat.id,
            threat: threat.threat,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainingObjectiveRest {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub objective: String,
    pub threats: Vec<ThreatRest>,
}

impl From<(TrainingObjective, Vec<ThreatRest>)> for TrainingObjectiveRest {
    fn from((objective, threats): (TrainingObjective, Vec<ThreatRest>)) -> Self {
        Self {
            id: objective.id,
            objective: objective.objective,
            threats,
        }
    }
}

pub type StructureWithElements = HashMap<
    Structure,
    (
        Vec<SkillRest>,
        Vec<StructureObjectiveRest>,
        Vec<WeaknessRest>,
    ),
>;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRest {
    pub id: Uuid,
    pub name: String,
    pub client_id: String,
    pub training_objectives: Vec<TrainingObjectiveRest>,
    pub structures: Vec<StructureRest>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<(Order, Vec<TrainingObjectiveRest>, StructureWithElements)> for OrderRest {
    fn from(
        (order, training_objectives, structures): (
            Order,
            Vec<TrainingObjectiveRest>,
            StructureWithElements,
        ),
    ) -> Self {
        let structures = structures
            .into_iter()
            .map(|(structure, elements)| (structure, elements).into())
            .collect();
        Self {
            id: order.id,
            name: order.name,
            client_id: order.client_id,
            training_objectives,
            structures,
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}
