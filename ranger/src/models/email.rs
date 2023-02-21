use lettre::Message;
use serde::{Deserialize, Serialize};

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
    pub fn create_message(&self) -> Message {
        Message::builder()
            .from(self.from_address.parse().unwrap())
            .to(self.to_address.parse().unwrap())
            .subject(self.subject.clone())
            .body(self.body.clone())
            .unwrap()
    }
}
