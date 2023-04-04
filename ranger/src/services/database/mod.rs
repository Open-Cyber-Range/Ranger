pub(crate) mod account;
pub(crate) mod condition;
pub(crate) mod deployment;
pub(crate) mod exercise;

use crate::{models::helpers::uuid::Uuid, utilities::run_migrations};
use actix::{Actor, Addr};
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::{
    dsl::now,
    helper_types::{AsSelect, Eq, Filter, Select, Update},
    mysql::{Mysql, MysqlConnection},
    query_builder::{InsertOrIgnoreStatement, InsertStatement},
    r2d2::{ConnectionManager, Pool, PooledConnection},
    sql_function, Insertable,
};

use super::websocket::WebSocketManager;

sql_function! (fn current_timestamp() -> Timestamp);

pub type All<Table, T> = Select<Table, AsSelect<T, Mysql>>;
pub type FilterExisting<Target, DeletedAtColumn> =
    Filter<Target, Eq<DeletedAtColumn, NaiveDateTime>>;
pub type ById<Id, R> = Filter<R, Eq<Id, Uuid>>;
pub type ByTemplateId<TemplateId, R> = Filter<R, Eq<TemplateId, Uuid>>;
pub type ByUsername<Username, R> = Filter<R, Eq<Username, String>>;
pub type SelectById<Table, Id, DeletedAtColumn, T> =
    ById<Id, FilterExisting<All<Table, T>, DeletedAtColumn>>;
pub type SelectByTemplateId<Table, TemplateId, DeletedAtColumn, T> =
    ByTemplateId<TemplateId, FilterExisting<All<Table, T>, DeletedAtColumn>>;
pub type SelectByTemplateIdAndUsername<Table, TemplateId, Username, DeletedAtColumn, T> =
    ByUsername<Username, ByTemplateId<TemplateId, FilterExisting<All<Table, T>, DeletedAtColumn>>>;
type UpdateDeletedAt<DeletedAtColumn> = Eq<DeletedAtColumn, now>;
pub type SoftDelete<L, DeletedAtColumn> = Update<L, UpdateDeletedAt<DeletedAtColumn>>;
pub type SoftDeleteById<Id, DeleteAtColumn, Table> = SoftDelete<ById<Id, Table>, DeleteAtColumn>;
pub type UpdateById<Id, DeletedAtColumn, Table, T> =
    Update<FilterExisting<ById<Id, Table>, DeletedAtColumn>, T>;
pub type Create<Type, Table> = InsertStatement<Table, <Type as Insertable<Table>>::Values>;
pub type CreateOrIgnore<Type, Table> =
    InsertOrIgnoreStatement<Table, <Type as Insertable<Table>>::Values>;

pub struct Database {
    websocket_manager_address: Addr<WebSocketManager>,
    connection_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Actor for Database {
    type Context = actix::Context<Self>;
}

impl Database {
    pub fn try_new(database_url: &str, websocket_manager: &Addr<WebSocketManager>) -> Result<Self> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let connection_pool = Pool::builder()
            .build(manager)
            .map_err(|error| anyhow!("Failed to create database connection pool: {}", error))?;
        let mut connection = connection_pool
            .get()
            .map_err(|error| anyhow!("Failed to get database connection: {}", error))?;

        run_migrations(&mut connection)
            .map_err(|error| anyhow!("Failed to run database migrations: {}", error))?;
        Ok(Self {
            connection_pool,
            websocket_manager_address: websocket_manager.clone(),
        })
    }

    pub fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
        Ok(self.connection_pool.get()?)
    }
}
