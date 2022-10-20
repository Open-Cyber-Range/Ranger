use super::Database;
use crate::models::helpers::uuid::Uuid;
use crate::models::{Deployment, NewDeployment};
use crate::schema::deployments;
use actix::{Handler, Message};
use anyhow::Result;
use diesel::{insert_into, RunQueryDsl};

#[derive(Message)]
#[rtype(result = "Result<Deployment>")]
pub struct CreateDeployment(pub NewDeployment);

impl Handler<CreateDeployment> for Database {
    type Result = Result<Deployment>;

    fn handle(&mut self, msg: CreateDeployment, _ctx: &mut Self::Context) -> Self::Result {
        let new_deployment = msg.0;
        let mut connection = self.get_connection()?;

        insert_into(deployments::table)
            .values(&new_deployment)
            .execute(&mut connection)?;
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
