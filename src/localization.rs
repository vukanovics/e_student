use crate::error::Error;
use rocket::{
    http::CookieJar,
    request::{FromRequest, Outcome},
    Request,
};
use serde::Serialize;

pub enum Language {
    English,
    SerbianLatin,
    SerbianCyrillic,
}

const LANGUAGE_CODE_ENGLISH: &'static str = "en";
const LANGUAGE_CODE_SERBIAN_LATIN: &'static str = "sr_latn";
const LANGUAGE_CODE_SERBIAN_CYRILLIC: &'static str = "sr_cyrl";

const DEFAULT_LANGUAGE: Language = Language::SerbianLatin;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Language {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = request.guard::<&CookieJar<'_>>().await.unwrap();
        match jar.get("language") {
            Some(language) => match language.value() {
                LANGUAGE_CODE_SERBIAN_LATIN => Outcome::Success(Self::SerbianLatin),
                LANGUAGE_CODE_SERBIAN_CYRILLIC => Outcome::Success(Self::SerbianCyrillic),
                LANGUAGE_CODE_ENGLISH => Outcome::Success(Self::English),
                _ => Outcome::Success(DEFAULT_LANGUAGE),
            },
            None => Outcome::Success(DEFAULT_LANGUAGE),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Localization {
    code: &'static str,
    username_or_email: &'static str,
    password: &'static str,
    stay_logged_in_for: &'static str,
    logged_in_as: &'static str,
    login: &'static str,
    minute: &'static str,
    minutes: &'static str,
    hour: &'static str,
    error_required_fields: &'static str,
    error_login_info: &'static str,
}

pub const LOCALIZATION_ENGLISH: Localization = Localization {
    code: LANGUAGE_CODE_ENGLISH,
    username_or_email: "Username or E-Mail",
    password: "Password",
    stay_logged_in_for: "Stay logged in for",
    logged_in_as: "Logged in as",
    login: "Login",
    minute: "minute",
    minutes: "minutes",
    hour: "hour",
    error_required_fields: "All fields are required!",
    error_login_info: "Invalid login info.",
};

pub const LOCALIZATION_SERBIAN_LATIN: Localization = Localization {
    code: LANGUAGE_CODE_SERBIAN_LATIN,
    username_or_email: "Korisničko ime ili E-Mejl",
    password: "Lozinka",
    stay_logged_in_for: "Ostani ulogovan",
    logged_in_as: "Ulogovan kao",
    login: "Uloguj se",
    minute: "minut",
    minutes: "minuta",
    hour: "sat",
    error_required_fields: "Sva polja su neophodna!",
    error_login_info: "Uneti podaci nisu validni.",
};

pub const LOCALIZATION_SERBIAN_CYRILLIC: Localization = Localization {
    code: LANGUAGE_CODE_SERBIAN_CYRILLIC,
    username_or_email: "Корисничко име или Е-Мејл",
    password: "Лозинка",
    stay_logged_in_for: "Остани улогован",
    logged_in_as: "Улогован као",
    login: "Улогуј се",
    minute: "минут",
    minutes: "минута",
    hour: "сат",
    error_required_fields: "Сва поља су неопходна!",
    error_login_info: "Унети подаци нису валидни.",
};
