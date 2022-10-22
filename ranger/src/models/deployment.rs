use crate::{
    constants::MAX_DEPLOYMENT_NAME_LENGTH,
    errors::RangerError,
    schema::deployments,
    services::database::{All, AllExisting, SelectById, SoftDeleteById},
    utilities::Validation,
};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

use super::helpers::uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = deployments)]
pub struct NewDeployment {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: String,
    pub deployment_group: Option<String>,
}

impl Validation for NewDeployment {
    fn validate(&self) -> Result<(), RangerError> {
        if self.name.len() > MAX_DEPLOYMENT_NAME_LENGTH {
            return Err(RangerError::DeploymentNameTooLong);
        }
        Ok(())
    }
}

#[derive(Queryable, Selectable, Clone, Debug, Deserialize, Serialize)]
#[diesel(table_name = deployments)]
pub struct Deployment {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: String,
    pub deployment_group: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Deployment {
    fn all_with_deleted() -> All<deployments::table, Self> {
        deployments::table.select(Deployment::as_select())
    }

    pub fn all() -> AllExisting<deployments::table, deployments::deleted_at, Self> {
        Self::all_with_deleted().filter(deployments::deleted_at.is_null())
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectById<deployments::table, deployments::id, deployments::deleted_at, Self> {
        Self::all().filter(deployments::id.eq(id))
    }

    pub fn soft_delete(
        id: Uuid,
    ) -> SoftDeleteById<deployments::id, deployments::deleted_at, deployments::table> {
        diesel::update(deployments::table.filter(deployments::id.eq(id)))
            .set(deployments::deleted_at.eq(diesel::dsl::now))
    }
}
