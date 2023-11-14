use super::helpers::uuid::Uuid;
use crate::{
    constants::{MAX_ORDER_NAME_LENGTH, NAIVEDATETIME_DEFAULT_VALUE},
    errors::RangerError,
    schema::{orders, threats, training_objectives},
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
use std::result::Result as StdResult;

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
        TrainingObjective::belonging_to(order).select(TrainingObjective::as_select())
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

    pub fn batch_insert(threats: Vec<Threat>) -> Create<Vec<Threat>, threats::table> {
        insert_into(threats::table).values(threats)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainingObjectiveRest {
    pub objective: String,
    pub threats: Vec<String>,
}

impl From<(TrainingObjective, Vec<Threat>)> for TrainingObjectiveRest {
    fn from((objective, threats): (TrainingObjective, Vec<Threat>)) -> Self {
        Self {
            objective: objective.objective,
            threats: threats.into_iter().map(|t| t.threat).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRest {
    pub id: Uuid,
    pub name: String,
    pub client_id: String,
    pub training_objectives: Vec<TrainingObjectiveRest>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<(Order, Vec<TrainingObjectiveRest>)> for OrderRest {
    fn from((order, training_objectives): (Order, Vec<TrainingObjectiveRest>)) -> Self {
        Self {
            id: order.id,
            name: order.name,
            client_id: order.client_id,
            training_objectives,
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}
