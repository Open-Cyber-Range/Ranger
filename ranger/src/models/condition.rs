use super::helpers::uuid::Uuid;
use crate::{
    schema::condition_messages,
    services::database::{All, Create, FilterExisting, SelectById, SoftDeleteById},
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    insert_into, ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = condition_messages)]
pub struct NewConditionMessage {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub condition_id: Uuid,
    pub value: BigDecimal,
}

impl NewConditionMessage {
    pub fn new(deployment_id: Uuid, condition_id: Uuid, value: BigDecimal) -> Self {
        Self {
            id: Uuid::random(),
            deployment_id,
            condition_id,
            value,
        }
    }

    pub fn create_insert(&self) -> Create<&Self, condition_messages::table> {
        insert_into(condition_messages::table).values(self)
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = condition_messages)]
pub struct ConditionMessage {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub condition_id: Uuid,
    pub value: BigDecimal,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl ConditionMessage {
    fn all_with_deleted() -> All<condition_messages::table, Self> {
        condition_messages::table.select(Self::as_select())
    }

    pub fn all(
    ) -> FilterExisting<All<condition_messages::table, Self>, condition_messages::deleted_at> {
        Self::all_with_deleted().filter(condition_messages::deleted_at.is_null())
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectById<
        condition_messages::table,
        condition_messages::id,
        condition_messages::deleted_at,
        Self,
    > {
        Self::all().filter(condition_messages::id.eq(id))
    }

    pub fn soft_delete(
        &self,
    ) -> SoftDeleteById<
        condition_messages::id,
        condition_messages::deleted_at,
        condition_messages::table,
    > {
        diesel::update(condition_messages::table.filter(condition_messages::id.eq(self.id)))
            .set(condition_messages::deleted_at.eq(diesel::dsl::now))
    }
}
