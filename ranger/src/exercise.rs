use actix::Message;
use anyhow::Result;
use sdl_parser::{parse_sdl, Scenario};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exercise {
    #[serde(default = "default_uuid")]
    pub uuid: Uuid,
    pub name: String,
    #[serde(deserialize_with = "deserialize_scenario")]
    pub scenario: Scenario,
}

impl Exercise {
    pub fn new(name: String, scenario: Scenario) -> Self {
        Self {
            uuid: default_uuid(),
            name,
            scenario,
        }
    }
}

fn default_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn deserialize_scenario<'de, D>(deserializer: D) -> Result<Scenario, D::Error>
where
    D: Deserializer<'de>,
{
    let schema_sdl = String::deserialize(deserializer).unwrap();
    match parse_sdl(&schema_sdl) {
        Ok(schema) => Ok(schema.scenario),
        Err(parsing_error) => Err(serde::de::Error::custom(format!(
            "Parse error {} for {}",
            parsing_error, schema_sdl
        ))),
    }
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct AddExercise(pub Exercise);

#[derive(Message, Debug)]
#[rtype(result = "Result<Exercise>")]
pub struct GetExercise(pub Uuid);
