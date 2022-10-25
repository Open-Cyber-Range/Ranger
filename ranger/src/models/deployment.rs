use crate::{
    constants::MAX_DEPLOYMENT_NAME_LENGTH,
    errors::RangerError,
    schema::{deployment_elements, deployments},
    services::database::{All, Create, FilterExisting, SelectById, SoftDeleteById},
    utilities::Validation,
};
use chrono::NaiveDateTime;
use diesel::{
    helper_types::{Eq, Filter, FindBy, Update},
    sql_types::Text,
    AsChangeset, AsExpression, ExpressionMethods, FromSqlRow, Identifiable, Insertable, QueryDsl,
    Queryable, Selectable, SelectableHelper,
};
use ranger_grpc::capabilities::DeployerTypes;
use serde::{Deserialize, Serialize};

use super::helpers::{deployer_type::DeployerType, uuid::Uuid};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = deployments)]
pub struct NewDeployment {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub name: String,
    pub sdl_schema: String,
    pub deployment_group: Option<String>,
}

impl NewDeployment {
    pub fn create_insert(&self) -> Create<&Self, deployments::table> {
        diesel::insert_into(deployments::table).values(self)
    }
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

#[derive(Clone, Debug, PartialEq, FromSqlRow, AsExpression, Eq, Deserialize, Serialize)]
#[diesel(sql_type = Text)]
pub enum ElementStatus {
    Ongoing,
    Success,
    Failed,
    Removed,
}

pub trait ScenarioReference
where
    Self: Send,
{
    fn reference(&self) -> String;
}

impl ScenarioReference for sdl_parser::common::Source {
    fn reference(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
}

impl ScenarioReference for String {
    fn reference(&self) -> String {
        self.clone()
    }
}

pub type BoxedScenarioReference = Box<dyn ScenarioReference>;

#[derive(
    Insertable,
    Identifiable,
    Queryable,
    Selectable,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    AsChangeset,
)]
#[diesel(table_name = deployment_elements)]
pub struct DeploymentElement {
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub scenario_reference: String,
    pub handler_reference: Option<String>,
    pub deployer_type: DeployerType,
    pub status: ElementStatus,
}

type ByDeploymentIdByScenarioReference<T> = Filter<
    Filter<
        FilterExisting<All<deployment_elements::table, T>, deployment_elements::deleted_at>,
        Eq<deployment_elements::deployment_id, Uuid>,
    >,
    Eq<deployment_elements::scenario_reference, String>,
>;

impl DeploymentElement {
    pub fn new(
        deployment_id: Uuid,
        reference_box: Box<dyn ScenarioReference>,
        deployer_type: DeployerTypes,
    ) -> Self {
        Self {
            id: Uuid::random(),
            deployment_id,
            scenario_reference: reference_box.reference(),
            handler_reference: None,
            deployer_type: DeployerType(deployer_type),
            status: ElementStatus::Ongoing,
        }
    }

    fn all_with_deleted() -> All<deployment_elements::table, Self> {
        deployment_elements::table.select(DeploymentElement::as_select())
    }

    pub fn all(
    ) -> FilterExisting<All<deployment_elements::table, Self>, deployment_elements::deleted_at>
    {
        Self::all_with_deleted().filter(deployment_elements::deleted_at.is_null())
    }

    pub fn by_id(
        id: Uuid,
    ) -> SelectById<
        deployment_elements::table,
        deployment_elements::id,
        deployment_elements::deleted_at,
        Self,
    > {
        Self::all().filter(deployment_elements::id.eq(id))
    }

    pub fn by_deployer_id_by_scenario_reference(
        deployment_id: Uuid,
        scenario_reference: BoxedScenarioReference,
    ) -> ByDeploymentIdByScenarioReference<Self> {
        Self::all()
            .filter(deployment_elements::deployment_id.eq(deployment_id))
            .filter(deployment_elements::scenario_reference.eq(scenario_reference.reference()))
    }

    pub fn create_insert(&self) -> Create<&Self, deployment_elements::table> {
        diesel::insert_into(deployment_elements::table).values(self)
    }

    pub fn create_update(
        &self,
    ) -> Update<FindBy<deployment_elements::table, deployment_elements::id, &Uuid>, &Self> {
        diesel::update(self).set(self)
    }
}

impl Deployment {
    fn all_with_deleted() -> All<deployments::table, Self> {
        deployments::table.select(Deployment::as_select())
    }

    pub fn all() -> FilterExisting<All<deployments::table, Self>, deployments::deleted_at> {
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
