use super::helpers::uuid::Uuid;
use crate::{
    errors::RangerError,
    schema::scenarios,
    services::database::{All, AllExisting, SelectById, SoftDeleteById},
    utilities::Validation,
};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use sdl_parser::Schema;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

#[derive(Insertable, Deserialize, Debug, Clone)]
#[diesel(table_name = scenarios)]
pub struct NewScenario {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub content: String,
}

impl Validation for NewScenario {
    fn validate(&self) -> StdResult<(), RangerError> {
        if Schema::from_yaml(self.content.as_str()).is_err() {
            return Err(RangerError::ScenarioParsingFailed);
        }
        Ok(())
    }
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone)]
#[diesel(table_name = scenarios)]
pub struct Scenario {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Scenario {
    fn all_with_deleted() -> All<scenarios::table, Self> {
        scenarios::table.select(Scenario::as_select())
    }

    pub fn all() -> AllExisting<scenarios::table, scenarios::deleted_at, Self> {
        Self::all_with_deleted().filter(scenarios::deleted_at.is_null())
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectById<scenarios::table, scenarios::id, scenarios::deleted_at, Self> {
        Self::all().filter(scenarios::id.eq(id))
    }

    pub fn soft_delete(
        id: Uuid,
    ) -> SoftDeleteById<scenarios::id, scenarios::deleted_at, scenarios::table> {
        diesel::update(scenarios::table.filter(scenarios::id.eq(id)))
            .set(scenarios::deleted_at.eq(diesel::dsl::now))
    }
}
