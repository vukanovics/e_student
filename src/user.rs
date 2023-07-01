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
    query_current,
    schema::users,
};

pub type UserId = u32;

#[derive(Clone, Debug, Queryable, Serialize, Insertable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: UserId,
    pub created: NaiveDateTime,
    pub password: String,
    pub email: String,
    pub account_type: AccountType,
    pub password_reset_required: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub last_login_time: Option<NaiveDateTime>,
    pub deleted: bool,
}

query_current!(User, users, UsersAlias, users_alias);

impl User {
    pub fn builder<'a>(email: &'a str, password: &'a str) -> UserBuilder<'a> {
        UserBuilder {
            email,
            password,
            account_type: AccountType::Student,
            password_reset_required: false,
            first_name: None,
            last_name: None,
            last_login_time: None,
        }
    }

    pub fn get_by_id(connection: &mut Connection, id: UserId) -> Result<Self, Error> {
        User::query_current()
            .filter(users::id.eq(id))
            .filter(users::deleted.eq(false))
            .limit(1)
            .first::<User>(connection)
            .map_err(Error::from)
    }

    pub fn get_by_email<'a>(
        connection: &mut diesel::MysqlConnection,
        email: &'a str,
    ) -> Result<User, Error> {
        User::query_current()
            .filter(users::email.eq(email))
            .filter(users::deleted.eq(false))
            .limit(1)
            .first::<User>(connection)
            .map_err(Error::from)
    }

    pub fn update_email<'a>(&mut self, email: &'a str) -> () {
        self.email = email.to_string();
    }

    pub fn update_account_type(&mut self, account_type: AccountType) -> () {
        self.account_type = account_type;
    }

    pub fn update_deleted(&mut self, deleted: bool) -> () {
        self.deleted = deleted;
    }

    pub fn update_database(&self, connection: &mut Connection) -> Result<(), Error> {
        diesel::insert_into(users::table)
            .values(self)
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

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub password: &'a str,
    pub email: &'a str,
    pub account_type: AccountType,
    pub password_reset_required: bool,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub last_login_time: Option<NaiveDateTime>,
}

impl NewUser<'_> {
    pub fn create(&self, connection: &mut Connection) -> Result<User, Error> {
        diesel::insert_into(users::table)
            .values(self)
            .execute(connection)?;
        User::get_by_email(connection, self.email)
    }
}

pub struct UserBuilder<'a> {
    email: &'a str,
    password: &'a str,
    account_type: AccountType,
    password_reset_required: bool,
    first_name: Option<&'a str>,
    last_name: Option<&'a str>,
    last_login_time: Option<NaiveDateTime>,
}

impl<'a> UserBuilder<'a> {
    pub fn with_account_type(mut self, account_type: AccountType) -> Self {
        self.account_type = account_type;
        self
    }

    pub fn build(self) -> NewUser<'a> {
        NewUser {
            password: self.password,
            email: self.email,
            account_type: self.account_type,
            password_reset_required: self.password_reset_required,
            first_name: self.first_name,
            last_name: self.last_name,
            last_login_time: self.last_login_time,
        }
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
                            .run(move |c| User::get_by_id(c, session.user))
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
