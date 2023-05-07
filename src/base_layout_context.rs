use crate::{
    error::Error,
    localization::{
        Language, Localization, LOCALIZATION_ENGLISH, LOCALIZATION_SERBIAN_CYRILLIC,
        LOCALIZATION_SERBIAN_LATIN,
    },
    models::User,
};
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct UserInfo {
    username: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct BaseLayoutContext {
    user_info: Option<UserInfo>,
    localization: Localization,
}

impl BaseLayoutContext {
    pub async fn new(language: Language, user: Option<User>) -> Result<Self, Error> {
        let user_info = match user {
            Some(user) => Some(UserInfo {
                username: user.username.unwrap_or(user.email),
            }),
            None => None,
        };

        let localization = match language {
            Language::English => LOCALIZATION_ENGLISH,
            Language::SerbianLatin => LOCALIZATION_SERBIAN_LATIN,
            Language::SerbianCyrillic => LOCALIZATION_SERBIAN_CYRILLIC,
        };

        Ok(Self {
            user_info,
            localization,
        })
    }
}
