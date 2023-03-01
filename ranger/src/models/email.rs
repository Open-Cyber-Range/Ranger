use lettre::{address::AddressError, Message};
use serde::{Deserialize, Serialize};

use super::helpers::uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailResource {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub to_address: String,
    pub subject: String,
    pub body: String,
}

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
    pub fn new(resource: EmailResource, from_address: String) -> Self {
        Self {
            id: resource.id,
            from_address,
            to_address: resource.to_address,
            subject: resource.subject,
            body: resource.body,
        }
    }

    pub fn create_message(&self) -> Result<Message, AddressError> {
        Ok(Message::builder()
            .from(self.from_address.parse()?)
            .to(self.to_address.parse()?)
            .subject(self.subject.clone())
            .body(self.body.clone())
            .unwrap())
    }
}
