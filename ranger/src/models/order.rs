use super::helpers::uuid::Uuid;
use crate::{
    constants::{MAX_ORDER_NAME_LENGTH, NAIVEDATETIME_DEFAULT_VALUE},
    errors::RangerError,
    schema::orders,
    services::database::{All, Create, FilterExisting, SelectById},
    utilities::Validation,
};
use chrono::NaiveDateTime;
use diesel::{
    insert_into,
    prelude::{Insertable, Queryable},
    ExpressionMethods, QueryDsl, Selectable, SelectableHelper,
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

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
