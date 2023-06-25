use crate::{error::Error, localization::Script, user::User};
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct UserInfo {
    username: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct BaseLayoutContext {
    user_info: UserInfo,
    script: Script,
}

impl BaseLayoutContext {
    pub async fn new(script: Script, user: &User) -> Result<Self, Error> {
        let user_info = UserInfo {
            username: user.username().unwrap_or(user.email()).to_string(),
        };

        Ok(Self {
            user_info,
            script
        })
    }
}
