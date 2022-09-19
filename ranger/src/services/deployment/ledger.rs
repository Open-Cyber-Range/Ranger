use actix::{Actor, Context, Handler, Message};
use anyhow::{anyhow, Result};
use sdl_parser::node::Source;
use std::collections::HashMap;

pub trait LedgerKey
where
    Self: Send,
{
    fn get_key(&self) -> String;
}

impl LedgerKey for Source {
    fn get_key(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
}

impl LedgerKey for String {
    fn get_key(&self) -> String {
        self.clone()
    }
}

pub struct Ledger {
    id_map: HashMap<String, String>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            id_map: HashMap::new(),
        }
    }
}

impl Actor for Ledger {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct CreateEntry(pub Box<dyn LedgerKey>, pub String);

impl Handler<CreateEntry> for Ledger {
    type Result = Result<()>;

    fn handle(
        &mut self,
        CreateEntry(key, id): CreateEntry,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.id_map.insert(key.get_key(), id);
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "Result<String>")]
pub struct GetEntry(pub Box<dyn LedgerKey>);

impl Handler<GetEntry> for Ledger {
    type Result = Result<String>;

    fn handle(&mut self, GetEntry(key): GetEntry, _ctx: &mut Self::Context) -> Self::Result {
        let id = self
            .id_map
            .get(&key.get_key())
            .ok_or_else(|| anyhow!("Key not found"))?;
        Ok(id.clone())
    }
}
