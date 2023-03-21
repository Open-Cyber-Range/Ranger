use lettre::{
    message::{header, SinglePart},
    Message,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::helpers::uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailResource {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub to_addresses: Vec<String>,
    pub reply_to_addresses: Option<Vec<String>>,
    pub cc_addresses: Option<Vec<String>>,
    pub bcc_addresses: Option<Vec<String>>,
    pub subject: String,
    pub body: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub from_address: String,
    pub to_addresses: Vec<String>,
    pub reply_to_addresses: Option<Vec<String>>,
    pub cc_addresses: Option<Vec<String>>,
    pub bcc_addresses: Option<Vec<String>>,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new(resource: EmailResource, from_address: String) -> Self {
        Self {
            id: resource.id,
            from_address,
            to_addresses: resource.to_addresses,
            reply_to_addresses: resource.reply_to_addresses,
            cc_addresses: resource.cc_addresses,
            bcc_addresses: resource.bcc_addresses,
            subject: resource.subject,
            body: resource.body,
        }
    }

    pub fn create_message(&self) -> Result<Message, Box<dyn Error>> {
        let mut message_builder = Message::builder()
            .from(self.from_address.parse()?)
            .subject(self.subject.clone());

        for to_address in self.to_addresses.clone() {
            message_builder = message_builder.to(to_address.trim().parse()?);
        }

        if let Some(reply_to_addresses) = &self.reply_to_addresses {
            for reply_to_address in reply_to_addresses.clone() {
                message_builder = message_builder.reply_to(reply_to_address.trim().parse()?);
            }
        }

        if let Some(cc_addresses) = &self.cc_addresses {
            for cc_address in cc_addresses.clone() {
                message_builder = message_builder.cc(cc_address.trim().parse()?);
            }
        }

        if let Some(bcc_addresses) = &self.bcc_addresses {
            for bcc_address in bcc_addresses.clone() {
                message_builder = message_builder.bcc(bcc_address.trim().parse()?);
            }
        }

        Ok(message_builder.singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(self.body.clone()),
        )?)
    }
}
