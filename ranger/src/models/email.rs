use crate::{
    schema::emails,
    services::database::{All, Create, DeleteById, SelectByIdFromAll},
};
use chrono::NaiveDateTime;
use diesel::{
    insert_into, ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable, SelectableHelper,
};
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

impl EmailResource {
    pub fn create_message(&self, from_address: String) -> Result<Message, Box<dyn Error>> {
        let mut message_builder = Message::builder()
            .from(from_address.parse()?)
            .subject(self.subject.clone());

        for to_address in self.to_addresses.clone() {
            if !to_address.trim().is_empty() {
                message_builder = message_builder.to(to_address.trim().parse()?);
            }
        }

        if let Some(reply_to_addresses) = &self.reply_to_addresses {
            for reply_to_address in reply_to_addresses.clone() {
                if !reply_to_address.trim().is_empty() {
                    message_builder = message_builder.reply_to(reply_to_address.trim().parse()?);
                }
            }
        }

        if let Some(cc_addresses) = &self.cc_addresses {
            for cc_address in cc_addresses.clone() {
                if !cc_address.trim().is_empty() {
                    message_builder = message_builder.cc(cc_address.trim().parse()?);
                }
            }
        }

        if let Some(bcc_addresses) = &self.bcc_addresses {
            for bcc_address in bcc_addresses.clone() {
                if !bcc_address.trim().is_empty() {
                    message_builder = message_builder.bcc(bcc_address.trim().parse()?);
                }
            }
        }

        Ok(message_builder.singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(self.body.clone()),
        )?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = emails)]
pub struct NewEmail {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub user_id: Option<String>,
    pub from_address: String,
    pub to_addresses: String,
    pub reply_to_addresses: Option<String>,
    pub cc_addresses: Option<String>,
    pub bcc_addresses: Option<String>,
    pub subject: String,
    pub body: String,
}

impl NewEmail {
    pub fn new(
        resource: EmailResource,
        from_address: String,
        exercise_id: Uuid,
        user_id: Option<String>,
    ) -> Self {
        Self {
            id: resource.id,
            exercise_id,
            user_id,
            from_address,
            to_addresses: resource.to_addresses.join(","),
            reply_to_addresses: resource
                .reply_to_addresses
                .map(|addresses| addresses.join(",")),
            cc_addresses: resource.cc_addresses.map(|addresses| addresses.join(",")),
            bcc_addresses: resource.bcc_addresses.map(|addresses| addresses.join(",")),
            subject: resource.subject,
            body: resource.body,
        }
    }

    pub fn create_insert(&self) -> Create<&Self, emails::table> {
        insert_into(emails::table).values(self)
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = emails)]
pub struct Email {
    #[serde(default = "Uuid::random")]
    pub id: Uuid,
    pub exercise_id: Uuid,
    pub user_id: Option<String>,
    pub from_address: String,
    pub to_addresses: String,
    pub reply_to_addresses: Option<String>,
    pub cc_addresses: Option<String>,
    pub bcc_addresses: Option<String>,
    pub subject: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

impl Email {
    pub fn all() -> All<emails::table, Self> {
        emails::table.select(Self::as_select())
    }

    pub fn by_id(id: Uuid) -> SelectByIdFromAll<emails::table, emails::id, Self> {
        Self::all().filter(emails::id.eq(id))
    }

    pub fn hard_delete(&self) -> DeleteById<emails::id, emails::table> {
        diesel::delete(emails::table.filter(emails::id.eq(self.id)))
    }
}
