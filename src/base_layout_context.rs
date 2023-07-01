use crate::{error::Error, localization::Script, user::User};
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct BaseLayoutContext {
    user: User,
    script: Script,
}

impl BaseLayoutContext {
    pub async fn new(script: Script, user: &User) -> Result<Self, Error> {
        Ok(Self {
            user: user.clone(),
            script
        })
    }
}
