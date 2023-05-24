use crate::error::Error;
use serde::Serialize;

pub enum Language {
    English,
    SerbianLatin,
    SerbianCyrillic,
}

const LANGUAGE_CODE_ENGLISH: &'static str = "en";
const LANGUAGE_CODE_SERBIAN_LATIN: &'static str = "sr_latn";
const LANGUAGE_CODE_SERBIAN_CYRILLIC: &'static str = "sr_cyrl";

impl Language {
    pub fn from_code<'a>(code: &'a str) -> Result<Language, Error> {
        match code {
            LANGUAGE_CODE_SERBIAN_LATIN => Ok(Self::SerbianLatin),
            LANGUAGE_CODE_SERBIAN_CYRILLIC => Ok(Self::SerbianCyrillic),
            LANGUAGE_CODE_ENGLISH => Ok(Self::English),
            _ => Err(Error::InvalidLanguageCode),
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
};
