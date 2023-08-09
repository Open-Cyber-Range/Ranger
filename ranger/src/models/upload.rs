use crate::constants::NAIVEDATETIME_DEFAULT_VALUE;
use crate::schema::artifacts;
use crate::services::database::All;
use crate::services::database::Create;
use crate::services::database::FilterExisting;
use crate::services::database::SelectById;
use crate::services::database::SelectByName;
use crate::services::database::SoftDeleteById;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::Queryable;
use diesel::Insertable;

use super::helpers::uuid::Uuid;

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Uuid,
    pub name: String,
    pub content: Vec<u8>,
    pub metric_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[diesel(table_name = artifacts)]
pub struct NewArtifact {
    pub id: Uuid,
    pub content: Vec<u8>,
    pub name: String,
}

impl NewArtifact {
    pub fn create_insert(&self) -> Create<&Self, artifacts::table> {
        insert_into(artifacts::table).values(self)
    }

    pub fn new(name: String, content: Vec<u8>) -> Self {
        Self {
            id: Uuid::random(),
            name,
            content,
        }
    }
}

impl Artifact {
    fn all_with_deleted() -> All<artifacts::table, Self> {
        artifacts::table.select(Self::as_select())
    }

    pub fn all() -> FilterExisting<All<artifacts::table, Self>, artifacts::deleted_at> {
        Self::all_with_deleted().filter(artifacts::deleted_at.eq(*NAIVEDATETIME_DEFAULT_VALUE))
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectById<artifacts::table, artifacts::id, artifacts::deleted_at, Self> {
        Self::all().filter(artifacts::id.eq(id))
    }


    pub fn soft_delete(
        &self,
    ) -> SoftDeleteById<artifacts::id, artifacts::deleted_at, artifacts::table> {
        diesel::update(artifacts::table.filter(artifacts::id.eq(self.id)))
            .set(artifacts::deleted_at.eq(diesel::dsl::now))
    }

    pub fn by_name(
        name: String,
    ) -> SelectByName<artifacts::table, artifacts::name, artifacts::deleted_at, Self> {
        Self::all().filter(artifacts::name.eq(name))
    }

}
