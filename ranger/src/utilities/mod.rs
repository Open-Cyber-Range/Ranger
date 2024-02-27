pub mod event;
pub mod scenario;
mod validation;

use crate::{
    constants::{
        DUPLICATE_ENTRY, FOREIGN_KEY_CONSTRAINT_FAILS, PACKAGE_CHECK_FAILED, RECORD_NOT_FOUND,
    },
    errors::RangerError,
};
use actix::MailboxError;
use anyhow::{anyhow, Result};
use diesel::mysql::Mysql;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::error;
pub use validation::*;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use std::path::Path;

pub fn create_mailbox_error_handler(actor_name: &str) -> impl Fn(MailboxError) -> RangerError + '_ {
    move |err| {
        error!("{} actor mailbox error: {}", actor_name, err);
        RangerError::ActixMailBoxError
    }
}

pub fn create_database_error_handler(
    action_name: &str,
) -> impl Fn(anyhow::Error) -> RangerError + '_ {
    move |err| {
        error!("{} error: {}", action_name, err);
        let error_string = format!("{err}");
        if error_string.contains(FOREIGN_KEY_CONSTRAINT_FAILS) {
            return RangerError::DatabaseConflict;
        } else if error_string.contains(RECORD_NOT_FOUND) {
            return RangerError::DatabaseRecordNotFound;
        } else if error_string.contains(DUPLICATE_ENTRY) {
            return RangerError::DatabaseConflict;
        }
        RangerError::DatabaseUnexpected
    }
}

pub fn create_package_error_handler(
    action_name: String,
    package_name: String,
) -> impl Fn(anyhow::Error) -> RangerError {
    move |err| {
        error!(
            "{} error for package '{}': {}",
            action_name, package_name, err
        );
        let error_string = format!("{err}");
        if error_string.contains(PACKAGE_CHECK_FAILED) {
            return RangerError::PackageCheckFailed(package_name.clone());
        }
        RangerError::DeputyQueryFailed
    }
}

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Mysql>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn try_some<T>(item: Option<T>, error_message: &str) -> Result<T> {
    item.ok_or_else(|| anyhow!("{:?}", error_message))
}

pub fn get_file_extension(filename: &str) -> Option<&str> {
    let path = Path::new(filename);
    path.extension().and_then(|extension| extension.to_str())
}
