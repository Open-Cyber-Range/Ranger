use lettre::{address::AddressError, Message};
use serde::{Deserialize, Serialize};

use super::helpers::uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailResource {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub to_address: String,
    pub reply_to_address: Option<String>,
    pub cc_address: Option<String>,
    pub bcc_address: Option<String>,
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
    pub reply_to_address: Option<String>,
    pub cc_address: Option<String>,
    pub bcc_address: Option<String>,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new(resource: EmailResource, from_address: String) -> Self {
        Self {
            id: resource.id,
            from_address,
            to_address: resource.to_address,
            reply_to_address: resource.reply_to_address,
            cc_address: resource.cc_address,
            bcc_address: resource.bcc_address,
            subject: resource.subject,
            body: resource.body,
        }
    }

    pub fn create_message(&self) -> Result<Message, AddressError> {
        let mut message_builder = Message::builder()
            .from(self.from_address.parse()?)
            .to(self.to_address.parse()?);

        if let Some(reply_to_address) = &self.reply_to_address {
            message_builder = message_builder.reply_to(reply_to_address.parse()?);
        }

        if let Some(cc_address) = &self.cc_address {
            message_builder = message_builder.cc(cc_address.parse()?);
        }

        if let Some(bcc_address) = &self.bcc_address {
            message_builder = message_builder.bcc(bcc_address.parse()?);
        }

        Ok(message_builder
            .subject(self.subject.clone())
            .body(self.body.clone())
            .unwrap())
    }
}
