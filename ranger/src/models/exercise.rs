use super::helpers::uuid::Uuid;
use crate::{
    constants::MAX_EXERCISE_NAME_LENGTH,
    errors::RangerError,
    schema::exercises,
    services::database::{All, Create, FilterExisting, SelectById, SoftDeleteById, UpdateById},
    utilities::Validation,
};
use chrono::NaiveDateTime;
use diesel::{
    insert_into, AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable,
    SelectableHelper,
};
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

#[derive(Insertable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = exercises)]
pub struct NewExercise {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: Option<String>,
}

impl NewExercise {
    pub fn create_insert(&self) -> Create<&Self, exercises::table> {
        insert_into(exercises::table).values(self)
    }
}

impl Validation for NewExercise {
    fn validate(&self) -> StdResult<(), RangerError> {
        if self.name.len() > MAX_EXERCISE_NAME_LENGTH {
            return Err(RangerError::ExerciseNameTooLong);
        }
        Ok(())
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

    pub fn all() -> FilterExisting<All<exercises::table, Self>, exercises::deleted_at> {
        Self::all_with_deleted().filter(exercises::deleted_at.is_null())
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectById<exercises::table, exercises::id, exercises::deleted_at, Self> {
        Self::all().filter(exercises::id.eq(id))
    }

    pub fn soft_delete(
        &self,
    ) -> SoftDeleteById<exercises::id, exercises::deleted_at, exercises::table> {
        diesel::update(exercises::table.filter(exercises::id.eq(self.id)))
            .set(exercises::deleted_at.eq(diesel::dsl::now))
    }
}

#[derive(AsChangeset, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[diesel(table_name = exercises)]
pub struct UpdateExercise {
    pub name: String,
    pub sdl_schema: Option<String>,
}

impl UpdateExercise {
    pub fn create_update(
        &self,
        id: Uuid,
    ) -> UpdateById<exercises::id, exercises::deleted_at, exercises::table, &Self> {
        diesel::update(exercises::table)
            .filter(exercises::id.eq(id))
            .filter(exercises::deleted_at.is_null())
            .set(self)
    }
}
