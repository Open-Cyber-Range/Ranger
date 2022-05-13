use actix::{Actor, Context, Handler, Message};
use anyhow::{anyhow, Result};
use sdl_parser::Scenario;
use std::collections::HashMap;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct AddScenario(pub(crate) Scenario);

#[derive(Message, Debug, PartialEq)]
#[rtype(result = "Result<Scenario>")]
pub struct GetScenario(pub(crate) String);

#[derive(Default, PartialEq)]
pub struct Database {
    scenarios: HashMap<String, Scenario>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Actor for Database {
    type Context = Context<Self>;
}

impl Handler<AddScenario> for Database {
    type Result = ();

    fn handle(&mut self, msg: AddScenario, _: &mut Context<Self>) -> Self::Result {
        self.scenarios.insert(msg.0.name.clone(), msg.0);
    }
}

impl Handler<GetScenario> for Database {
    type Result = Result<Scenario>;

    fn handle(&mut self, msg: GetScenario, _: &mut Context<Self>) -> Self::Result {
        match self.scenarios.get(&msg.0) {
            Some(scenario) => Ok(scenario.to_owned()),
            None => Err(anyhow!("Scenario not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{AddScenario, Database, GetScenario};
    use actix::Actor;
    use anyhow::Result;
    use sdl_parser::test::TEST_SCHEMA;

    #[test]
    fn add_test_exercise() -> Result<()> {
        let system = actix::System::new();
        system.block_on(async {
            let database_address = Database::new().start();
            let result = database_address
                .send(AddScenario(TEST_SCHEMA.scenario.clone()))
                .await;
            assert!(result.is_ok());
        });
        Ok(())
    }

    #[test]
    fn get_test_exercise() -> Result<()> {
        let system = actix::System::new();
        let result = system.block_on(async {
            let database_address = Database::new().start();
            database_address
                .send(AddScenario(TEST_SCHEMA.scenario.clone()))
                .await
                .unwrap();
            let result = database_address
                .send(GetScenario("test-scenario".to_string()))
                .await;
            result?
        })?;
        assert_eq!(TEST_SCHEMA.scenario, result);
        Ok(())
    }
}
