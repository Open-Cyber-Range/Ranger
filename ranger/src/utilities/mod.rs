mod validation;

use actix::MailboxError;
use log::error;
use uuid::Uuid;
pub use validation::*;

use crate::errors::RangerError;

pub fn default_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn create_mailbox_error_handler(actor_name: &str) -> impl Fn(MailboxError) -> RangerError + '_ {
    move |err| {
        error!("{} actor mailbox error: {}", actor_name, err);
        RangerError::ActixMailBoxError
    }
}
