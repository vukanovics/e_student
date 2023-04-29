use crate::{database::Database, error::Error};
use chrono::{DateTime, Duration, Utc};
use log::info;
use rocket::http::{Cookie, CookieJar};
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
    pub async fn new(database: &Database, jar: &CookieJar<'_>) -> Result<Self, Error> {
        let session_key = jar
            .get_pending("session_key")
            .or(jar.get("session_key").cloned())
            .map(|c| hex::decode(c.value().to_owned()))
            .transpose()?;

        let user_info = match session_key {
            Some(session_key) => {
                let session = database
                    .run(move |c| Database::get_session_by_key(c, session_key))
                    .await?;

                let now = Utc::now();
                let session_expires = DateTime::<Utc>::from_utc(session.last_refreshed, Utc)
                    + Duration::seconds(session.timeout_duration_seconds as i64);

                match now < session_expires {
                    true => {
                        let user = database
                            .run(move |c| Database::get_user_by_id(c, session.user_id))
                            .await?;
                        Some(UserInfo {
                            username: user.username.unwrap_or(user.email),
                        })
                    }
                    false => {
                        info!("User has supplied an expired session_key cookie, removing it");
                        jar.remove(Cookie::new("session_key", ""));
                        None
                    }
                }
            }
            None => None,
        };

        Ok(Self { user_info })
    }
}
