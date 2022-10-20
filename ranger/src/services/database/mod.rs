pub(crate) mod deployment;
pub(crate) mod exercise;
pub(crate) mod scenario;

use crate::models::helpers::uuid::Uuid;
use actix::Actor;
use anyhow::{anyhow, Result};
use diesel::{
    dsl::now,
    helper_types::{AsSelect, Eq, Filter, IsNull, Select, Update},
    mysql::{Mysql, MysqlConnection},
    r2d2::{ConnectionManager, Pool, PooledConnection},
    sql_function,
};

sql_function! (fn current_timestamp() -> Timestamp);

pub type All<Table, T> = Select<Table, AsSelect<T, Mysql>>;
pub type AllExisting<Table, DeletedAtColumn, T> = Filter<All<Table, T>, IsNull<DeletedAtColumn>>;
pub type ById<Id, R> = Filter<R, Eq<Id, Uuid>>;
pub type SelectById<Table, Id, DeletedAtColumn, T> =
    ById<Id, AllExisting<Table, DeletedAtColumn, T>>;
type UpdateDeletedAt<DeletedAtColumn> = Eq<DeletedAtColumn, now>;
pub type SoftDeleteById<Id, DeleteAtColumn, Table> =
    Update<ById<Id, Table>, UpdateDeletedAt<DeleteAtColumn>>;

pub struct Database {
    connection_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Actor for Database {
    type Context = actix::Context<Self>;
}

impl Database {
    pub fn try_new(database_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        Ok(Self {
            connection_pool: Pool::builder()
                .build(manager)
                .map_err(|error| anyhow!("Failed to create database connection pool: {}", error))?,
        })
    }

    pub fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
        Ok(self.connection_pool.get()?)
    }
}
