use super::helpers::uuid::Uuid;
use crate::{
    constants::DELETED_AT_DEFAULT_VALUE,
    schema::scores,
    services::database::{All, Create, FilterExisting, SelectById, SoftDeleteById, UpdateById},
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    helper_types::{Eq, Filter},
    insert_into, AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable,
    SelectableHelper,
};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = scores)]
pub struct NewScore {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub deployment_id: Uuid,
    pub tlo_name: String,
    pub metric_name: String,
    pub value: BigDecimal,
}

impl NewScore {
    pub fn new(
        exercise_id: Uuid,
        deployment_id: Uuid,
        tlo_name: String,
        metric_name: String,
        value: BigDecimal,
    ) -> Self {
        Self {
            id: Uuid::random(),
            exercise_id,
            deployment_id,
            tlo_name,
            metric_name,
            value,
        }
    }

    pub fn create_insert(&self) -> Create<&Self, scores::table> {
        insert_into(scores::table).values(self)
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = scores)]
pub struct Score {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub deployment_id: Uuid,
    pub tlo_name: String,
    pub metric_name: String,
    pub value: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}

type ByDeploymentId<T> =
    Filter<FilterExisting<T, scores::deleted_at>, Eq<scores::deployment_id, Uuid>>;

type ByDeploymentIdByTloNameByMetricName<T> = Filter<
    Filter<ByDeploymentId<All<scores::table, T>>, Eq<scores::tlo_name, String>>,
    Eq<scores::metric_name, String>,
>;

impl Score {
    fn all_with_deleted() -> All<scores::table, Self> {
        scores::table.select(Self::as_select())
    }

    pub fn all() -> FilterExisting<All<scores::table, Self>, scores::deleted_at> {
        Self::all_with_deleted().filter(scores::deleted_at.eq(*DELETED_AT_DEFAULT_VALUE))
    }

    pub fn by_id(id: Uuid) -> SelectById<scores::table, scores::id, scores::deleted_at, Self> {
        Self::all().filter(scores::id.eq(id))
    }

    pub fn by_deployment_id_by_tlo_name_by_metric_name(
        deployment_id: Uuid,
        tlo_name: String,
        metric_name: String,
    ) -> ByDeploymentIdByTloNameByMetricName<Self> {
        Self::all()
            .filter(scores::deployment_id.eq(deployment_id))
            .filter(scores::tlo_name.eq(tlo_name))
            .filter(scores::metric_name.eq(metric_name))
    }

    pub fn soft_delete(&self) -> SoftDeleteById<scores::id, scores::deleted_at, scores::table> {
        diesel::update(scores::table.filter(scores::id.eq(self.id)))
            .set(scores::deleted_at.eq(diesel::dsl::now))
    }
}

#[derive(AsChangeset, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = scores)]
pub struct UpdateScore {
    id: Uuid,
    value: BigDecimal,
}

impl UpdateScore {
    pub fn create_update(
        &self,
        id: Uuid,
    ) -> UpdateById<scores::id, scores::deleted_at, scores::table, &Self> {
        diesel::update(scores::table)
            .filter(scores::id.eq(id))
            .filter(scores::deleted_at.eq(*DELETED_AT_DEFAULT_VALUE))
            .set(self)
    }
}
