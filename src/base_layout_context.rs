use crate::{error::Error, models::User};
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct UserInfo {
    username: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct BaseLayoutContext {
    user_info: Option<UserInfo>,
}

impl BaseLayoutContext {
    pub async fn new(user: Option<User>) -> Result<Self, Error> {
        let user_info = match user {
            Some(user) => Some(UserInfo {
                username: user.username.unwrap_or(user.email),
            }),
            None => None,
        };

        Ok(Self { user_info })
    }
}
