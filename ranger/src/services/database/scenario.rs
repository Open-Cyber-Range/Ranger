use super::Database;
use crate::models::helpers::uuid::Uuid;
use crate::models::{NewScenario, Scenario};
use crate::schema::scenarios;
use actix::{Handler, Message};
use anyhow::Result;
use diesel::{insert_into, RunQueryDsl};

#[derive(Message)]
#[rtype(result = "Result<Scenario>")]
pub struct CreateScenario(pub NewScenario);

impl Handler<CreateScenario> for Database {
    type Result = Result<Scenario>;

    fn handle(&mut self, msg: CreateScenario, _ctx: &mut Self::Context) -> Self::Result {
        let new_scenario = msg.0;
        let mut connection = self.get_connection()?;

        insert_into(scenarios::table)
            .values(&new_scenario)
            .execute(&mut connection)?;
        let scenario = Scenario::by_id(new_scenario.id).first(&mut connection)?;

        Ok(scenario)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Scenario>")]
pub struct GetScenario(pub Uuid);

impl Handler<GetScenario> for Database {
    type Result = Result<Scenario>;

    fn handle(&mut self, msg: GetScenario, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = msg.0;
        let mut connection = self.get_connection()?;

        let scenario = Scenario::by_id(uuid).first(&mut connection)?;

        Ok(scenario)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Uuid>")]
pub struct DeleteScenario(pub Uuid);

impl Handler<DeleteScenario> for Database {
    type Result = Result<Uuid>;

    fn handle(&mut self, msg: DeleteScenario, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = msg.0;
        let mut connection = self.get_connection()?;

        Scenario::by_id(uuid).first(&mut connection)?;
        Scenario::soft_delete(uuid).execute(&mut connection)?;

        Ok(uuid)
    }
}
