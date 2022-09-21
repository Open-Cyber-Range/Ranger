use crate::models::{AddExercise, Exercise, GetExercise};
use actix::{Actor, Context, Handler};
use anyhow::{anyhow, Result};
use std::collections::{hash_map::Entry::Vacant, HashMap};
use uuid::Uuid;

#[derive(Default, PartialEq, Eq)]
pub struct Database {
    exercises: HashMap<Uuid, Exercise>,
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
        match self.exercises.entry(msg.0.uuid) {
            Vacant(entry) => {
                entry.insert(msg.0);
            }
            _ => log::error!("This exercise already exists in the database"),
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
    use rand::Rng;
    use sdl_parser::{test::TEST_SCHEMA, Scenario};

    fn create_test_exercise(scenario: Scenario) -> Exercise {
        let mut rng = rand::thread_rng();
        let random_num: u8 = rng.gen_range(0..100);
        let exercise_name = "test_exercise_".to_string() + &random_num.to_string();
        Exercise::new(exercise_name, scenario)
    }

    #[test]
    fn add_test_exercise() -> Result<()> {
        let system = System::new();
        let exercise = create_test_exercise(TEST_SCHEMA.scenario.clone());

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
        let exercise = create_test_exercise(TEST_SCHEMA.scenario.clone());

        let result = system.block_on(async {
            let database_address = Database::new().start();
            database_address
                .send(AddExercise(exercise.clone()))
                .await
                .unwrap();
            let result = database_address.send(GetExercise(exercise.uuid)).await;
            result?
        })?;
        assert_eq!(exercise, result);
        Ok(())
    }
}
