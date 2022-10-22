use super::helpers::uuid::Uuid;
use crate::{
    constants::MAX_EXERCISE_NAME_LENGTH,
    errors::RangerError,
    schema::exercises,
    services::database::{All, AllExisting, SelectById, SoftDeleteById},
    utilities::Validation,
};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

#[derive(Insertable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[diesel(table_name = exercises)]
pub struct NewExercise {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: Option<String>,
}

impl Validation for NewExercise {
    fn validate(&self) -> StdResult<(), RangerError> {
        if self.name.len() > MAX_EXERCISE_NAME_LENGTH {
            return Err(RangerError::ExeciseNameTooLong);
        }
        Ok(())
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[diesel(table_name = exercises)]
pub struct Exercise {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Exercise {
    fn all_with_deleted() -> All<exercises::table, Self> {
        exercises::table.select(Self::as_select())
    }

    pub fn all() -> AllExisting<exercises::table, exercises::deleted_at, Self> {
        Self::all_with_deleted().filter(exercises::deleted_at.is_null())
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectById<exercises::table, exercises::id, exercises::deleted_at, Self> {
        Self::all().filter(exercises::id.eq(id))
    }

    pub fn soft_delete(
        id: Uuid,
    ) -> SoftDeleteById<exercises::id, exercises::deleted_at, exercises::table> {
        diesel::update(exercises::table.filter(exercises::id.eq(id)))
            .set(exercises::deleted_at.eq(diesel::dsl::now))
    }
}
