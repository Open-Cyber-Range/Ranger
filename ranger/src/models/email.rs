use lettre::Message;
use serde::{Deserialize, Serialize};

use crate::errors::RangerError;

use super::helpers::uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn create_message(&self) -> Result<Message, RangerError> {
        let from_address = match self.from_address.parse() {
            Ok(from_address) => from_address,
            Err(_) => return Err(RangerError::EmailMessageCreationFailed),
        };

        let to_address = match self.to_address.parse() {
            Ok(to_address) => to_address,
            Err(_) => return Err(RangerError::EmailMessageCreationFailed),
        };

        Ok(Message::builder()
            .from(from_address)
            .to(to_address)
            .subject(self.subject.clone())
            .body(self.body.clone())
            .unwrap())
    }
}
