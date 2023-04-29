use chrono::Utc;
use rand::{Fill, SeedableRng};
use rocket::{
    form::{Form, FromForm},
    get,
    http::{Cookie, CookieJar, Status},
    post,
};

use serde::Serialize;

use rocket_dyn_templates::{context, Template};

use crate::{
    base_layout_context::BaseLayoutContext, database::Database, error::Error, models::Session,
};

#[derive(Serialize, Debug)]
struct LoginLayoutContext {
    base_layout_context: BaseLayoutContext,
    error_message: Option<String>,
    success_message: Option<String>,
}

impl LoginLayoutContext {
    pub fn new(database: &Database, jar: &CookieJar) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(database, jar)?,
            error_message: None,
            success_message: None,
        })
    }

    pub fn with_error_message(mut self, error_message: Option<String>) -> Self {
        self.error_message = error_message;
        self
    }

    pub fn with_success_message(mut self, success_message: Option<String>) -> Self {
        self.success_message = success_message;
        self
    }
}

#[get("/login")]
pub fn get() -> Result<Template, Status> {
    Ok(Template::render("routes/login", context! {}))
}

#[derive(FromForm, Debug)]
#[allow(unused)]
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

#[post("/login", data = "<form>")]
pub async fn post<'a>(
    database: Database,
    jar: &'a CookieJar<'a>,
    form: Form<LoginFormData>,
) -> Result<Template, Status> {
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
            Err(Error::Diesel(diesel::result::Error::NotFound)) => {
                break 'requirements Some("Username or e-mail not recognized.");
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        if !bcrypt::verify(&form.password, &user.password).map_err(|e| Error::Bcrypt(e))? {
            break 'requirements Some("Invalid password!");
        }

        fn generate_session_key() -> Result<[u8; 32], Error> {
            let mut session_key = [0u8; 32];
            let mut rng = rand::rngs::StdRng::from_entropy();
            session_key.try_fill(&mut rng).map_err(|e| Error::Rand(e))?;
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
        return Ok(Template::render(
            "routes/login",
            LoginLayoutContext::new(&database, jar)?
                .with_error_message(Some(error_message.to_string())),
        ));
    }
    return Ok(Template::render(
        "routes/login",
        LoginLayoutContext::new(&database, jar)?
            .with_success_message(Some("Successfully logged in!".to_string())),
    ));
}
