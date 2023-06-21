use crate::error::Error;
use handlebars::Handlebars;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::{smtp::authentication::Credentials, stub::AsyncStubTransport},
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use log::{error, info};
use rocket::fairing::Fairing;
use rocket::{async_trait, fairing, http::uncased::UncasedStr, Build, Rocket};
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
    // it's an option because of the custom Drop mechanics,
    // it will always be Some during Mail's lifetime
    sender: Option<Box<dyn MailSender + Send + Sync + 'static>>,
    handlebars: Handlebars<'static>,
}

impl Mail {
    pub fn new<T>(sender: T) -> Result<Self, Error>
    where
        T: MailSender + Send + Sync + 'static,
    {
        let mut handlebars = Handlebars::new();
        // if a value is missing from the context, fail
        handlebars.set_strict_mode(true);

        handlebars.register_template_file("invite", "templates/mail/invite.html.hbs")?;
        Ok(Self {
            sender: Some(Box::new(sender)),
            handlebars,
        })
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

        self.sender.as_ref().unwrap().send(message).await
    }

    pub fn fairing() -> MailFairing {
        MailFairing {}
    }
}

impl Drop for Mail {
    fn drop(&mut self) {
        let sender = self.sender.take();
        // at this point, the rocket tokio runtime is already down
        // however, lettre needs one in order to drop the sender
        // TODO: this likely has problems
        rocket::tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .spawn_blocking(move || drop(sender));
    }
}

pub struct SmtpMailSender {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpMailSender {
    pub fn new() -> Result<SmtpMailSender, Error> {
        let credentials = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_PASSWORD.to_owned());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(SMTP_ADDRESS)?
            .credentials(credentials)
            .build();
        Ok(SmtpMailSender { transport })
    }
}

#[async_trait]
impl MailSender for SmtpMailSender {
    async fn send(&self, message: Message) -> Result<(), Error> {
        self.transport
            .send(message)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }
}

pub struct StubMailSender {
    transport: AsyncStubTransport,
}

impl StubMailSender {
    pub fn new() -> Result<StubMailSender, Error> {
        Ok(StubMailSender {
            transport: AsyncStubTransport::new_ok(),
        })
    }
}

#[async_trait]
impl MailSender for StubMailSender {
    async fn send(&self, message: Message) -> Result<(), Error> {
        // AsyncStubTransport made with new_ok always succeeds
        info!("Sending a stub email: {:?}", message);
        Ok(self.transport.send(message).await.map(|_| ()).unwrap())
    }
}

pub struct MailFairing {}

#[async_trait]
impl Fairing for MailFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "MailFairing",
            kind: rocket::fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let figment = rocket.figment();

        let mail = if figment.profile().as_str() == UncasedStr::new("debug") {
            Mail::new(StubMailSender::new().unwrap())
        } else {
            Mail::new(SmtpMailSender::new().unwrap())
        };

        match mail {
            Ok(mail) => rocket::fairing::Result::Ok(rocket.manage(mail)),
            Err(e) => {
                error!("Failed to start mailing fairing: {:?}", e);
                rocket::fairing::Result::Err(rocket)
            }
        }
    }
}
