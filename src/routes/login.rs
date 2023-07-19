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
    database::Database,
    error::Error,
    localization::Script,
    models::Session,
    user::{User, SESSION_KEY_COOKIE_NAME},
};

#[derive(Clone, Serialize, Debug)]
struct LoginLayoutContext {
    script: Script,
    show_error_all_fields_required: bool,
    show_error_invalid_login_info: bool,
}

impl LoginLayoutContext {
    pub async fn new(script: Script) -> Result<Self, Error> {
        Ok(Self {
            script,
            show_error_all_fields_required: false,
            show_error_invalid_login_info: false,
        })
    }

    pub fn show_error_all_fields_required(mut self) -> Self {
        self.show_error_all_fields_required = true;
        self
    }

    pub fn show_error_invalid_login_info(mut self) -> Self {
        self.show_error_invalid_login_info = true;
        self
    }
}

#[get("/login", rank = 2)]
pub async fn get(language: Script) -> Result<Template, Status> {
    let context = LoginLayoutContext::new(language).await?;
    Ok(Template::render("routes/login", context))
}

#[get("/login", rank = 1)]
pub async fn get_logged_in(_user: &User) -> Redirect {
    Redirect::to("/courses")
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
    language: Script,
) -> Result<LoginResponse, Status> {
    if !form.all_fields_populated() {
        return Ok(LoginResponse::Failure(Template::render(
            "routes/login",
            LoginLayoutContext::new(language)
                .await?
                .show_error_all_fields_required(),
        )));
    }

    let user = match {
        let username_or_email = form.username_or_email.clone();
        database
            .run(move |c| User::get_by_email(c, &username_or_email))
            .await
    } {
        Ok(user) => user,
        Err(Error::DatabaseEntryNotFound) => {
            return Ok(LoginResponse::Failure(Template::render(
                "routes/login",
                LoginLayoutContext::new(language)
                    .await?
                    .show_error_invalid_login_info(),
            )));
        }
        Err(e) => {
            return Err(e.into());
        }
    };

    if !bcrypt::verify(&form.password, &user.password).map_err(Error::from)? {
        return Ok(LoginResponse::Failure(Template::render(
            "routes/login",
            LoginLayoutContext::new(language)
                .await?
                .show_error_invalid_login_info(),
        )));
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
        user: user.id,
        created_on: session_start.naive_utc(),
        last_refreshed: session_start.naive_utc(),
        timeout_duration_seconds: form.timeout,
    };

    database
        .run(move |c| Database::insert_session(c, &session))
        .await?;

    let mut cookie = Cookie::new(SESSION_KEY_COOKIE_NAME, hex::encode(session_key));

    cookie.set_secure(true);
    cookie.set_http_only(true);

    jar.add(cookie);

    Ok(LoginResponse::Success(Redirect::to(uri!("/courses"))))
}
