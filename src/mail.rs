use crate::error::Error;
use handlebars::Handlebars;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};
use rocket::async_trait;
use serde::Serialize;

const SMTP_ADDRESS: &str = dotenvy_macro::dotenv!("SMTP_ADDRESS");
const SMTP_USERNAME: &str = dotenvy_macro::dotenv!("SMTP_USERNAME");
const SMTP_PASSWORD: &str = dotenvy_macro::dotenv!("SMTP_PASSWORD");

const SMTP_FROM_ADDRESS: &str = dotenvy_macro::dotenv!("SMTP_FROM_ADDRESS");

#[async_trait]
pub trait MailSender {
    async fn send(&self, message: Message) -> Result<(), Error>;
}

pub struct Mail {
    transport: SmtpTransport,
    handlebars: Handlebars<'static>,
}

impl Mail {
    pub fn new() -> Result<Self, Error> {
        let mut handlebars = Handlebars::new();
        // if a value is missing from the context, fail
        handlebars.set_strict_mode(true);

        handlebars.register_template_file("invite", "templates/mail/invite.html.hbs")?;

        let credentials = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_PASSWORD.to_owned());
        let transport = SmtpTransport::relay(SMTP_ADDRESS)?
            .credentials(credentials)
            .build();

        Ok(Self {
            transport,
            handlebars,
        })
    }

    async fn send(&self, message: &Message) -> Result<(), Error> {
        self.transport
            .send(message)
            .map(|_| ())
            .map_err(Error::from)
    }

    #[allow(unused)]
    pub async fn send_invite(&self, to: Address, temporary_password: &str) -> Result<(), Error> {
        let from_address: Address = SMTP_FROM_ADDRESS.parse().unwrap();

        #[derive(Serialize)]
        pub struct InviteMailContext<'a> {
            temporary_password: &'a str,
        }

        let message = Message::builder()
            .from(Mailbox::new(None, from_address))
            .to(Mailbox::new(None, to))
            .subject("You've been invited to the ASSZS eStudent platform")
            .header(ContentType::TEXT_HTML)
            .body(
                self.handlebars
                    .render("invite", &InviteMailContext { temporary_password })?,
            )
            .unwrap();

        self.send(&message).await
    }
}
