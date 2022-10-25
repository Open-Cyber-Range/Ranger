mod validation;

use crate::{
    constants::{FOREIGN_KEY_CONSTRAINT_FAILS, RECORD_NOT_FOUND},
    errors::RangerError,
};
use actix::MailboxError;
use log::error;
pub use validation::*;

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
        let error_string = format!("{}", err);
        if error_string.contains(FOREIGN_KEY_CONSTRAINT_FAILS) {
            return RangerError::DatabaseConflict;
        } else if error_string.contains(RECORD_NOT_FOUND) {
            return RangerError::DatabaseRecordNotFound;
        }
        RangerError::DatabaseUnexpected
    }
}
