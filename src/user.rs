use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use log::info;
use rocket::{
    http::{Cookie, CookieJar, Status},
    outcome::try_outcome,
    request::{FromRequest, Outcome},
    Request,
};
use serde::Serialize;

use crate::{
    database::{Connection, Database},
    error::Error,
    models::{AccountType, Session},
};

pub type UserId = u32;

#[derive(Clone, Debug, Queryable, Serialize)]
#[allow(unused)]
pub struct User {
    pub id: UserId,
    pub password: String,
    pub email: String,
    pub account_type: AccountType,
    pub password_reset_required: bool,
    pub username: Option<String>,
    pub last_login_time: Option<NaiveDateTime>,
}

impl User {
    pub fn get_by_id(connection: &mut Connection, id: UserId) -> Result<Self, Error> {
        use crate::schema::users;
        users::table
            .filter(users::id.eq(id))
            .limit(1)
            .first::<User>(connection)
            .map_err(Error::from)
    }

    pub fn get_by_username_or_email<'a>(
        connection: &mut diesel::MysqlConnection,
        username_or_email: &'a str,
    ) -> Result<User, Error> {
        use crate::schema::users;
        users::table
            .filter(
                users::username
                    .eq(username_or_email)
                    .or(users::email.eq(username_or_email)),
            )
            .limit(1)
            .first::<User>(connection)
            .map_err(Error::from)
    }

    pub fn delete(&self, connection: &mut diesel::MysqlConnection) -> Result<(), Error> {
        use crate::schema::users;
        diesel::delete(users::table.filter(users::id.eq(self.id)))
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }

    pub fn account_type(&self) -> AccountType {
        self.account_type
    }

    pub fn is_professor(&self) -> bool {
        self.account_type == AccountType::Professor || self.is_administrator()
    }

    pub fn is_administrator(&self) -> bool {
        self.account_type == AccountType::Administrator
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_ref().map(|x| &**x)
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r User {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_result: &Result<Option<User>, Error> = request
            .local_cache_async(async {
                let jar = request.guard::<&CookieJar>().await.unwrap();
                let session_key = jar
                    .get_pending("session_key")
                    .or(jar.get("session_key").cloned())
                    .map(|c| hex::decode(c.value().to_owned()))
                    .transpose()
                    .map_err(Error::Hex)?;

                let database = request.guard::<Database>().await.unwrap();
                if let Some(session_key) = session_key {
                    let session: Session = database
                        .run(move |c| Database::get_session_by_key(c, session_key))
                        .await?;

                    let now = Utc::now();
                    let session_expires = DateTime::<Utc>::from_utc(session.last_refreshed, Utc)
                        + Duration::seconds(session.timeout_duration_seconds as i64);

                    if now < session_expires {
                        let user = database
                            .run(move |c| User::get_by_id(c, session.user_id))
                            .await?;
                        return Ok(Some(user));
                    }

                    info!("User has supplied an expired session_key cookie, removing it");
                    jar.remove(Cookie::new("session_key", ""));
                }
                Ok(None)
            })
            .await;
        match user_result {
            Ok(Some(user)) => Outcome::Success(user),
            Ok(None) => Outcome::Forward(()),
            Err(e) => Outcome::Failure((Status::InternalServerError, e.clone())),
        }
    }
}

pub struct Professor<'r>(pub &'r User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Professor<'r> {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<&User>().await);
        if user.is_professor() {
            Outcome::Success(Professor(user))
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct Administrator<'r>(pub &'r User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Administrator<'r> {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<&User>().await);
        if user.is_administrator() {
            Outcome::Success(Administrator(user))
        } else {
            Outcome::Forward(())
        }
    }
}
