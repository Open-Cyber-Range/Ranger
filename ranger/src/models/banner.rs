use super::helpers::uuid::Uuid;
use crate::{
    schema::banners,
    services::database::{All, Create, HardUpdateById},
};
use chrono::NaiveDateTime;
use diesel::{
    helper_types::{Eq, Filter},
    insert_into,
    query_builder::{DeleteStatement, IntoUpdateTarget},
    AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = banners)]
pub struct Banner {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Banner {
    pub fn all() -> All<banners::table, Self> {
        banners::table.select(Self::as_select())
    }

    pub fn by_id(id: Uuid) -> Filter<All<banners::table, Self>, Eq<banners::id, Uuid>> {
        Self::all().filter(banners::id.eq(id))
    }

    pub fn by_name(name: String) -> Filter<All<banners::table, Self>, Eq<banners::name, String>> {
        Self::all().filter(banners::name.eq(name))
    }

    pub fn hard_delete(
        &self,
    ) -> Filter<
        DeleteStatement<banners::table, <banners::table as IntoUpdateTarget>::WhereClause>,
        Eq<banners::id, Uuid>,
    > {
        diesel::delete(banners::table).filter(banners::id.eq(self.id))
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
    pub name: String,
    pub content: String,
}

impl UpdateBanner {
    pub fn create_update(&self, id: Uuid) -> HardUpdateById<banners::id, banners::table, &Self> {
        diesel::update(banners::table)
            .filter(banners::id.eq(id))
            .set(self)
    }
}
