use lettre::{message::MultiPart, Message};
use serde::{Deserialize, Serialize};
use std::error::Error;

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

    pub fn create_message(&self) -> Result<Message, Box<dyn Error>> {
        let mut message_builder = Message::builder()
            .from(self.from_address.parse()?)
            .subject(self.subject.clone());

        for to_address in self.to_address.clone().split(',') {
            message_builder = message_builder.to(to_address.trim().parse()?);
        }

        if self.reply_to_address.is_some() && !self.reply_to_address.clone().unwrap().is_empty() {
            for reply_to_address in self.reply_to_address.clone().unwrap().split(',') {
                message_builder = message_builder.reply_to(reply_to_address.trim().parse()?);
            }
        }

        if self.cc_address.is_some() && !self.cc_address.clone().unwrap().is_empty() {
            for cc_address in self.cc_address.clone().unwrap().split(',') {
                message_builder = message_builder.cc(cc_address.trim().parse()?);
            }
        }

        if self.bcc_address.is_some() && !self.bcc_address.clone().unwrap().is_empty() {
            for bcc_address in self.bcc_address.clone().unwrap().split(',') {
                message_builder = message_builder.bcc(bcc_address.trim().parse()?);
            }
        }

        Ok(message_builder.multipart(MultiPart::alternative_plain_html(
            String::from("Hello from OCR!"),
            self.body.clone(),
        ))?)
    }
}
