use super::Database;
use crate::models::helpers::uuid::Uuid;
use crate::models::{Deployment, DeploymentElement, NewDeployment, ScenarioReference};
use actix::{Handler, Message};
use anyhow::Result;
use diesel::RunQueryDsl;

#[derive(Message)]
#[rtype(result = "Result<Deployment>")]
pub struct CreateDeployment(pub NewDeployment);

impl Handler<CreateDeployment> for Database {
    type Result = Result<Deployment>;

    fn handle(&mut self, msg: CreateDeployment, _ctx: &mut Self::Context) -> Self::Result {
        let new_deployment = msg.0;
        let mut connection = self.get_connection()?;

        new_deployment.create_insert().execute(&mut connection)?;
        let deployment = Deployment::by_id(new_deployment.id).first(&mut connection)?;

        Ok(deployment)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Deployment>")]
pub struct GetDeployment(pub Uuid);

impl Handler<GetDeployment> for Database {
    type Result = Result<Deployment>;

    fn handle(&mut self, msg: GetDeployment, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let mut connection = self.get_connection()?;

        let deployment = Deployment::by_id(id).first(&mut connection)?;
        Ok(deployment)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid>")]
pub struct DeleteDeployment(pub Uuid);

impl Handler<DeleteDeployment> for Database {
    type Result = Result<Uuid>;

    fn handle(&mut self, msg: DeleteDeployment, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let mut connection = self.get_connection()?;

        Deployment::by_id(id).first(&mut connection)?;
        Deployment::soft_delete(id).execute(&mut connection)?;

        Ok(id)
    }
}

#[derive(Message)]
#[rtype(result = "Result<DeploymentElement>")]
pub struct CreateDeploymentElement(pub DeploymentElement);

impl Handler<CreateDeploymentElement> for Database {
    type Result = Result<DeploymentElement>;

    fn handle(&mut self, msg: CreateDeploymentElement, _ctx: &mut Self::Context) -> Self::Result {
        let new_deployment_element = msg.0;
        let mut connection = self.get_connection()?;

        new_deployment_element
            .create_insert()
            .execute(&mut connection)?;
        let deployment_element =
            DeploymentElement::by_id(new_deployment_element.id).first(&mut connection)?;

        Ok(deployment_element)
    }
}

#[derive(Message)]
#[rtype(result = "Result<DeploymentElement>")]
pub struct UpdateDeploymentElement(pub DeploymentElement);

impl Handler<UpdateDeploymentElement> for Database {
    type Result = Result<DeploymentElement>;

    fn handle(&mut self, msg: UpdateDeploymentElement, _ctx: &mut Self::Context) -> Self::Result {
        let new_deployment_element = msg.0;
        let mut connection = self.get_connection()?;

        let updated_rows = new_deployment_element
            .create_update()
            .execute(&mut connection)?;
        if updated_rows != 1 {
            return Err(anyhow::anyhow!("Deployment element not found"));
        }
        let deployment_element =
            DeploymentElement::by_id(new_deployment_element.id).first(&mut connection)?;

        Ok(deployment_element)
    }
}

#[derive(Message)]
#[rtype(result = "Result<DeploymentElement>")]
pub struct GetDeploymentElementByDeploymentIdByScenarioReference(
    pub Uuid,
    pub Box<dyn ScenarioReference>,
);

impl Handler<GetDeploymentElementByDeploymentIdByScenarioReference> for Database {
    type Result = Result<DeploymentElement>;

    fn handle(
        &mut self,
        msg: GetDeploymentElementByDeploymentIdByScenarioReference,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let deployment_id = msg.0;
        let scenario_reference = msg.1;
        let mut connection = self.get_connection()?;

        let deployment_element = DeploymentElement::by_deployer_id_by_scenario_reference(
            deployment_id,
            scenario_reference,
        )
        .first(&mut connection)?;

        Ok(deployment_element)
    }
}
