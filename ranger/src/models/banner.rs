use super::helpers::uuid::Uuid;
use crate::{
    constants::NAIVEDATETIME_DEFAULT_VALUE,
    schema::banners,
    services::database::{
        All, Create, FilterExisting, SelectById, SelectByName, SoftDeleteById, UpdateById,
    },
};
use chrono::NaiveDateTime;
use diesel::{
    insert_into, AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable,
    SelectableHelper,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = banners)]
pub struct Banner {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}

impl Banner {
    fn all_with_deleted() -> All<banners::table, Self> {
        banners::table.select(Self::as_select())
    }

    pub fn all() -> FilterExisting<All<banners::table, Self>, banners::deleted_at> {
        Self::all_with_deleted().filter(banners::deleted_at.eq(*NAIVEDATETIME_DEFAULT_VALUE))
    }

    pub fn by_id(id: Uuid) -> SelectById<banners::table, banners::id, banners::deleted_at, Self> {
        Self::all().filter(banners::id.eq(id))
    }

    pub fn by_name(
        name: String,
    ) -> SelectByName<banners::table, banners::name, banners::deleted_at, Self> {
        Self::all().filter(banners::name.eq(name))
    }

    pub fn soft_delete(&self) -> SoftDeleteById<banners::id, banners::deleted_at, banners::table> {
        diesel::update(banners::table.filter(banners::id.eq(self.id)))
            .set(banners::deleted_at.eq(diesel::dsl::now))
    }
}

#[derive(Insertable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = banners)]
pub struct NewBanner {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub content: String,
}

impl NewBanner {
    pub fn create_insert(&self) -> Create<&Self, banners::table> {
        insert_into(banners::table).values(self)
    }
}

#[derive(AsChangeset, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = banners)]
pub struct UpdateBanner {
    pub id: Uuid,
    pub name: String,
    pub content: String,
}

impl UpdateBanner {
    pub fn create_update(
        &self,
        id: Uuid,
    ) -> UpdateById<banners::id, banners::deleted_at, banners::table, &Self> {
        diesel::update(banners::table)
            .filter(banners::id.eq(id))
            .filter(banners::deleted_at.eq(*NAIVEDATETIME_DEFAULT_VALUE))
            .set(self)
    }
}
