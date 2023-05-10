use chrono::Utc;
use rand::{Fill, SeedableRng};
use rocket::{
    form::{Form, FromForm},
    get,
    http::{Cookie, CookieJar, Status},
    post,
    response::Redirect,
    uri, Responder,
};

use serde::Serialize;

use rocket_dyn_templates::Template;

use crate::{
    base_layout_context::BaseLayoutContext, database::Database, error::Error,
    localization::Language, models::Session, user::User,
};

#[derive(Clone, Serialize, Debug)]
struct LoginLayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    error_message: Option<String>,
}

impl LoginLayoutContext {
    pub async fn new(language: Language, user: Option<&User>) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            error_message: None,
        })
    }

    pub fn with_error_message(mut self, error_message: Option<String>) -> Self {
        self.error_message = error_message;
        self
    }
}

#[get("/login")]
pub async fn get(language: Language) -> Result<Template, Status> {
    Ok(Template::render(
        "routes/login",
        LoginLayoutContext::new(language, None).await?,
    ))
}

#[derive(FromForm, Debug)]
pub struct LoginFormData {
    username_or_email: String,
    password: String,
    timeout: u32,
}

impl LoginFormData {
    pub fn all_fields_populated(&self) -> bool {
        !self.username_or_email.is_empty() && !self.password.is_empty()
    }
}

#[derive(Responder)]
pub enum LoginResponse {
    Success(Redirect),
    Failure(Template),
}

#[post("/login", data = "<form>")]
pub async fn post(
    database: Database,
    jar: &CookieJar<'_>,
    form: Form<LoginFormData>,
    language: Language,
) -> Result<LoginResponse, Status> {
    if let Some(error_message) = 'requirements: {
        if !form.all_fields_populated() {
            break 'requirements Some("All fields are required!");
        }

        let user = match {
            let username_or_email = form.username_or_email.clone();
            database
                .run(move |c| Database::get_user_by_username_or_email(c, &username_or_email))
                .await
        } {
            Ok(user) => user,
            Err(Error::DatabaseEntryNotFound) => {
                break 'requirements Some("Username or e-mail not recognized.");
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        if !bcrypt::verify(&form.password, &user.password).map_err(Error::from)? {
            break 'requirements Some("Invalid password!");
        }

        fn generate_session_key() -> Result<[u8; 32], Error> {
            let mut session_key = [0u8; 32];
            let mut rng = rand::rngs::StdRng::from_entropy();
            session_key.try_fill(&mut rng).map_err(Error::from)?;
            Ok(session_key)
        }

        let session_key = generate_session_key()?;

        let session_start = Utc::now();

        let session = Session {
            session_key: session_key.to_vec(),
            user_id: user.id,
            created_on: session_start.naive_utc(),
            last_refreshed: session_start.naive_utc(),
            timeout_duration_seconds: form.timeout,
        };

        database
            .run(move |c| Database::insert_session(c, &session))
            .await?;

        let mut cookie = Cookie::new("session_key", hex::encode(session_key));

        cookie.set_secure(true);
        cookie.set_http_only(true);

        jar.add(cookie);

        None
    } {
        return Ok(LoginResponse::Failure(Template::render(
            "routes/login",
            LoginLayoutContext::new(language, None)
                .await?
                .with_error_message(Some(error_message.to_string())),
        )));
    }

    Ok(LoginResponse::Success(Redirect::to(uri!("/overview"))))
}
