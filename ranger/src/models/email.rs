use lettre::{address::AddressError, message::MultiPart, Message};
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
            .subject(self.subject.clone());

        let to_addresses: Vec<&str> = self.to_address.split(',').collect();
        for to_address in to_addresses {
            message_builder = message_builder.to(to_address.trim().parse()?);
        }

        if self.reply_to_address.is_some() && !self.reply_to_address.clone().unwrap().is_empty() {
            message_builder =
                message_builder.reply_to(self.reply_to_address.clone().unwrap().parse()?);
        }

        if self.cc_address.is_some() && !self.cc_address.clone().unwrap().is_empty() {
            message_builder = message_builder.cc(self.cc_address.clone().unwrap().parse()?);
        }

        if self.bcc_address.is_some() && !self.bcc_address.clone().unwrap().is_empty() {
            message_builder = message_builder.bcc(self.bcc_address.clone().unwrap().parse()?);
        }

        Ok(message_builder
            .multipart(MultiPart::alternative_plain_html(
                String::from("Hello from OCR!"),
                self.body.clone(),
            ))
            .unwrap())
    }
}
