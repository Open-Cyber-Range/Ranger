use actix::{Actor, Context, Handler, Message};
use anyhow::{anyhow, Result};
use rand::Rng;
use sdl_parser::{parse_sdl, Scenario};
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    collections::{hash_map::Entry::Vacant, HashMap},
};
use uuid::Uuid;

fn default_uuid() -> Vec<u8> {
    Uuid::new_v4().into_bytes().to_vec()
}

pub fn deserialize<'de, D>(d: D) -> Result<Scenario, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d).unwrap();
    match parse_sdl(&s) {
        Ok(schema) => Ok(schema.scenario),
        Err(e) => Err(serde::de::Error::custom(format!("Parse error {} for {}", e, s))),
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exercise {
    #[serde(default = "default_uuid")]
    uuid: Vec<u8>,
    name: String,
    #[serde(deserialize_with = "deserialize")]
    pub scenario: Scenario,
}

impl Exercise {
    pub fn new(name: String, scenario: Scenario) -> Self {
        Self {
            uuid: (default_uuid()),
            name: (name),
            scenario: (scenario),
        }
    }

    pub fn create_test_exercise(scenario: Scenario) -> Self {
        let mut rng = rand::thread_rng();
        let random_num: u8 = rng.gen_range(0..100);
        let exercise_name = "test_exercise_".to_string() + &random_num.to_string();
        Exercise::new(exercise_name, scenario)
    }
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct AddExercise(pub Exercise);

#[derive(Message, Debug)]
#[rtype(result = "Result<Exercise>")]
pub struct GetExercise(pub String);

#[derive(Default, PartialEq)]
pub struct Database {
    exercises: HashMap<String, Exercise>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Actor for Database {
    type Context = Context<Self>;
}

impl Handler<AddExercise> for Database {
    type Result = ();

    fn handle(&mut self, msg: AddExercise, _: &mut Context<Self>) -> Self::Result {
        if let Vacant(e) = self.exercises.entry(msg.0.name.clone()) {
            e.insert(msg.0);
        } else {
            log::error!("This exercise already exists in the database");
        }
    }
}

impl Handler<GetExercise> for Database {
    type Result = Result<Exercise>;

    fn handle(&mut self, msg: GetExercise, _: &mut Context<Self>) -> Self::Result {
        match self.exercises.get(&msg.0) {
            Some(exercise) => Ok(exercise.to_owned()),
            None => Err(anyhow!("Exercise not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{AddExercise, Database, Exercise, GetExercise};
    use actix::{Actor, System};
    use anyhow::Result;
    use sdl_parser::test::TEST_SCHEMA;

    #[test]
    fn add_test_exercise() -> Result<()> {
        let system = System::new();
        let exercise = Exercise::create_test_exercise(TEST_SCHEMA.scenario.clone());

        system.block_on(async {
            let database_address = Database::new().start();
            let result = database_address.send(AddExercise(exercise)).await;
            assert!(result.is_ok());
        });
        Ok(())
    }

    #[test]
    fn get_test_exercise() -> Result<()> {
        let system = System::new();
        let exercise = Exercise::create_test_exercise(TEST_SCHEMA.scenario.clone());

        let result = system.block_on(async {
            let database_address = Database::new().start();
            database_address
                .send(AddExercise(exercise.clone()))
                .await
                .unwrap();
            let result = database_address
                .send(GetExercise(exercise.name.clone()))
                .await;
            result?
        })?;
        assert_eq!(exercise, result);
        Ok(())
    }
}
