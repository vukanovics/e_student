use bcrypt::DEFAULT_COST;
use lettre::Address;
use rand::{distributions::Alphanumeric, Rng};
use rocket::{form::Form, get, http::Status, post, FromForm, State};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    localization::Script,
    mail::Mail,
    models::AccountType,
    user::{Administrator, User},
};

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    show_success_message: bool,
    show_invalid_email: bool,
    show_duplicate_email: bool,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            show_success_message: false,
            show_duplicate_email: false,
            show_invalid_email: false,
        })
    }

    pub fn success(mut self) -> Self {
        self.show_success_message = true;
        self
    }

    pub fn duplicate_email(mut self) -> Self {
        self.show_duplicate_email = true;
        self
    }

    pub fn invalid_email(mut self) -> Self {
        self.show_invalid_email = true;
        self
    }
}

#[get("/users/create", rank = 0)]
pub async fn get(language: Script, administrator: Administrator<'_>) -> Result<Template, Status> {
    let user = administrator.0;
    Ok(Template::render(
        "routes/administrator/users/create",
        LayoutContext::new(language, user).await?,
    ))
}

#[derive(FromForm, Debug)]
pub struct FormData {
    email: String,
}

#[post("/users/create", data = "<form>", rank = 0)]
pub async fn post(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    mail: &State<Mail>,
    form: Form<FormData>,
) -> Result<Template, Status> {
    let user = administrator.0;
    let address = match Address::try_from(form.email.clone()) {
        Ok(address) => address,
        Err(_) => {
            return Ok(Template::render(
                "routes/administrator/users/create",
                LayoutContext::new(language, user)
                    .await?
                    .invalid_email(),
            ))
        }
    };

    let plain_password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    {
        let password = bcrypt::hash(plain_password.clone(), DEFAULT_COST).map_err(Error::from)?;
        match database
            .run(move |c| {
                Database::create_new_user(c, &form.email, &password, AccountType::Student)
            })
            .await
        {
            Ok(_) => (),
            Err(Error::DatabaseDuplicateEntry) => {
                return Ok(Template::render(
                    "routes/administrator/users/create",
                    LayoutContext::new(language, user)
                        .await?
                        .duplicate_email(),
                ))
            }
            Err(e) => return Err(e.into()),
        }
    }

    mail.send_invite(address, &plain_password).await?;

    Ok(Template::render(
        "routes/administrator/users/create",
        LayoutContext::new(language, user).await?.success(),
    ))
}
