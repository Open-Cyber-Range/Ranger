use anyhow::{anyhow, Ok, Result};
use core::result::Result::Ok as MailerOk;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use crate::configuration::MailerConfiguration;

pub struct Mailer {}

impl Mailer {
    pub fn send_mail(configuration: MailerConfiguration, recipient: String) -> Result<()> {
        let email = Message::builder()
            .from(configuration.username.parse().unwrap())
            .to(recipient.parse().unwrap())
            .subject("Greetings from OCR!")
            .body(String::from("Hi!"))
            .unwrap();

        let creds = Credentials::new(configuration.username, configuration.password);

        let mailer = SmtpTransport::starttls_relay(&configuration.server_address)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            MailerOk(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}
