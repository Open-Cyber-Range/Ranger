pub(crate) mod deployment;
pub(crate) mod exercise;

use crate::models::helpers::uuid::Uuid;
use actix::Actor;
use anyhow::{anyhow, Result};
use diesel::{
    dsl::now,
    helper_types::{AsSelect, Eq, Filter, IsNull, Select, Update},
    mysql::{Mysql, MysqlConnection},
    query_builder::InsertStatement,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    sql_function, Insertable,
};

sql_function! (fn current_timestamp() -> Timestamp);

//Target All<Table, T>

pub type All<Table, T> = Select<Table, AsSelect<T, Mysql>>;
pub type FilterExisting<Target, DeletedAtColumn> = Filter<Target, IsNull<DeletedAtColumn>>;
pub type ById<Id, R> = Filter<R, Eq<Id, Uuid>>;
pub type SelectById<Table, Id, DeletedAtColumn, T> =
    ById<Id, FilterExisting<All<Table, T>, DeletedAtColumn>>;
type UpdateDeletedAt<DeletedAtColumn> = Eq<DeletedAtColumn, now>;
pub type SoftDeleteById<Id, DeleteAtColumn, Table> =
    Update<ById<Id, Table>, UpdateDeletedAt<DeleteAtColumn>>;
pub type UpdateById<Id, DeletedAtColumn, Table, T> =
    Update<FilterExisting<ById<Id, Table>, DeletedAtColumn>, T>;
pub type Create<Type, Table> = InsertStatement<Table, <Type as Insertable<Table>>::Values>;

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
