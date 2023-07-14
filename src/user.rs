use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    prelude::*,
    serialize::ToSql,
    sql_types::{TinyInt, Unsigned},
    AsExpression, FromSqlRow,
};
use log::info;
use rocket::{
    form::FromFormField,
    http::{Cookie, CookieJar, Status},
    outcome::try_outcome,
    request::{FromRequest, Outcome},
    Request,
};
use serde::Serialize;

use crate::{
    database::{Connection, Database, SortDirection},
    error::Error,
    index::{Generation, Index, IndexNumber, Program},
    models::Session,
    schema::{generations, indicies, programs, users},
};

#[repr(u8)]
#[derive(FromFormField, AsExpression, FromSqlRow, Serialize, PartialEq, Debug, Clone, Copy)]
#[diesel(sql_type = Unsigned<TinyInt>)]
pub enum AccountType {
    Student = 0,
    Professor = 1,
    Administrator = 2,
}

impl<DB: Backend> FromSql<Unsigned<TinyInt>, DB> for AccountType
where
    u8: FromSql<Unsigned<TinyInt>, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Self::try_from(u8::from_sql(bytes)?).map_err(|_| "Invalid AccountType value".into())
    }
}

impl<DB: Backend> ToSql<Unsigned<TinyInt>, DB> for AccountType
where
    u8: ToSql<Unsigned<TinyInt>, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Self::Student => 0.to_sql(out),
            Self::Professor => 1.to_sql(out),
            Self::Administrator => 2.to_sql(out),
        }
    }
}

impl TryFrom<u8> for AccountType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccountType::Student),
            1 => Ok(AccountType::Professor),
            2 => Ok(AccountType::Administrator),
            _ => Err(Error::InvalidAccountTypeValue),
        }
    }
}

pub type UserId = u32;

#[derive(Clone, Debug, Queryable, Serialize, Insertable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: UserId,
    pub password: String,
    pub email: String,
    pub account_type: AccountType,
    pub password_reset_required: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub last_login_time: Option<NaiveDateTime>,
    pub deleted: bool,
}

#[derive(Serialize, Debug)]
pub struct IndexGenerationProgram {
    index: Index,
    generation: Generation,
    program: Program,
}

#[derive(Serialize, Debug)]
pub struct UserWithIndex {
    #[serde(flatten)]
    pub user: User,
    #[serde(flatten)]
    pub index: Option<IndexGenerationProgram>,
}

impl User {
    pub fn builder<'a>(email: String, password: String) -> UserBuilder {
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
        users::table
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
        users::table
            .filter(users::email.eq(email))
            .filter(users::deleted.eq(false))
            .limit(1)
            .first::<User>(connection)
            .map_err(Error::from)
    }

    pub fn update_email<'a>(
        &self,
        connection: &mut Connection,
        email: &'a str,
    ) -> Result<(), Error> {
        diesel::update(self)
            .set(users::email.eq(email))
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn update_account_type(
        &self,
        connection: &mut Connection,
        account_type: AccountType,
    ) -> Result<(), Error> {
        diesel::update(self)
            .set(users::account_type.eq(account_type))
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn update_deleted(&self, connection: &mut Connection, deleted: bool) -> Result<(), Error> {
        diesel::update(self)
            .set(users::deleted.eq(deleted))
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn update_first_name<'a>(
        &self,
        connection: &mut Connection,
        first_name: Option<&'a str>,
    ) -> Result<(), Error> {
        diesel::update(self)
            .set(users::first_name.eq(first_name))
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn update_last_name<'a>(
        &self,
        connection: &mut Connection,
        last_name: Option<&'a str>,
    ) -> Result<(), Error> {
        diesel::update(self)
            .set(users::last_name.eq(last_name))
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
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
    pub fn create(&self, connection: &mut Connection) -> Result<(), Error> {
        diesel::insert_into(users::table)
            .values(self)
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }
}

pub struct UserBuilder {
    email: String,
    password: String,
    account_type: AccountType,
    password_reset_required: bool,
    first_name: Option<String>,
    last_name: Option<String>,
    last_login_time: Option<NaiveDateTime>,
}

impl UserBuilder {
    pub fn with_first_name(mut self, first_name: Option<String>) -> Self {
        self.first_name = first_name;
        self
    }

    pub fn with_last_name(mut self, last_name: Option<String>) -> Self {
        self.last_name = last_name;
        self
    }

    pub fn with_account_type(mut self, account_type: AccountType) -> Self {
        self.account_type = account_type;
        self
    }

    pub fn build<'a>(&'a self) -> NewUser<'a> {
        NewUser {
            password: &self.password,
            email: &self.email,
            account_type: self.account_type,
            password_reset_required: self.password_reset_required,
            first_name: self.first_name.as_deref(),
            last_name: self.last_name.as_deref(),
            last_login_time: self.last_login_time,
        }
    }
}

#[derive(Debug)]
pub struct UsersRetrievalOptions {
    pub filter_email: Option<String>,
    pub filter_account_type: Option<AccountType>,
    pub filter_first_name: Option<String>,
    pub filter_last_name: Option<String>,
    pub filter_program: Option<String>,
    pub filter_generation: Option<u32>,
    pub filter_index_number: Option<IndexNumber>,

    pub sort_by_email: Option<SortDirection>,
    pub sort_by_account_type: Option<SortDirection>,
    pub sort_by_first_name: Option<SortDirection>,
    pub sort_by_last_name: Option<SortDirection>,
    // sorts by program, then generation, and finally the index number
    pub sort_by_index: Option<SortDirection>,
}

impl UsersRetrievalOptions {
    pub fn new() -> Self {
        Self {
            filter_email: None,
            filter_account_type: None,
            filter_first_name: None,
            filter_last_name: None,
            filter_program: None,
            filter_generation: None,
            filter_index_number: None,
            sort_by_email: None,
            sort_by_account_type: None,
            sort_by_first_name: None,
            sort_by_last_name: None,
            sort_by_index: None,
        }
    }
}

pub struct Users(pub Vec<UserWithIndex>);
impl Users {
    pub fn get_all(
        connection: &mut Connection,
        options: UsersRetrievalOptions,
    ) -> Result<Users, Error> {
        let mut query = users::table
            .left_join(
                indicies::table
                    .inner_join(generations::table)
                    .inner_join(programs::table),
            )
            .filter(users::deleted.eq(false))
            .into_boxed();

        if let Some(filter) = options.filter_email {
            query = query.filter(users::email.like(format!("%{}%", filter)))
        }

        if let Some(filter) = options.filter_account_type {
            query = query.filter(users::account_type.eq(filter))
        }

        if let Some(filter) = options.filter_first_name {
            query = query.filter(users::first_name.like(format!("%{}%", filter)))
        }

        if let Some(filter) = options.filter_last_name {
            query = query.filter(users::last_name.like(format!("%{}%", filter)))
        }

        if let Some(filter) = options.filter_program {
            query = query.filter(programs::short_name.like(format!("%{}%", filter)))
        }

        if let Some(filter) = options.filter_generation {
            query = query.filter(generations::year.eq(filter))
        }

        if let Some(filter) = options.filter_index_number {
            query = query.filter(indicies::number.eq(filter))
        }

        if let Some(order) = options.sort_by_first_name {
            query = match order {
                SortDirection::Ascending => query.then_order_by(users::first_name.asc()),
                SortDirection::Descending => query.then_order_by(users::first_name.desc()),
            };
        }

        if let Some(order) = options.sort_by_last_name {
            query = match order {
                SortDirection::Ascending => query.then_order_by(users::last_name.asc()),
                SortDirection::Descending => query.then_order_by(users::last_name.desc()),
            };
        }

        if let Some(order) = options.sort_by_email {
            query = match order {
                SortDirection::Ascending => query.then_order_by(users::email.asc()),
                SortDirection::Descending => query.then_order_by(users::email.desc()),
            };
        }

        if let Some(order) = options.sort_by_account_type {
            query = match order {
                SortDirection::Ascending => query.then_order_by(users::account_type.asc()),
                SortDirection::Descending => query.then_order_by(users::account_type.desc()),
            };
        }

        if let Some(order) = options.sort_by_index {
            query = match order {
                SortDirection::Ascending => query
                    .then_order_by(programs::short_name.asc())
                    .then_order_by(generations::year.asc())
                    .then_order_by(indicies::number.asc()),
                SortDirection::Descending => query
                    .then_order_by(programs::short_name.desc())
                    .then_order_by(generations::year.desc())
                    .then_order_by(indicies::number.desc()),
            };
        }

        query
            .load::<(User, Option<(Index, Generation, Program)>)>(connection)
            .map(|mut users| {
                let users = users
                    .drain(..)
                    .map(|(user, index)| {
                        let index =
                            index.map(|(index, generation, program)| IndexGenerationProgram {
                                index,
                                generation,
                                program,
                            });
                        UserWithIndex { user, index }
                    })
                    .collect();
                Users { 0: users }
            })
            .map_err(Error::from)
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
