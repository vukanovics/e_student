use crate::schema::{sessions, users};
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    sql_types::{TinyInt, Unsigned},
    AsExpression, FromSqlRow, Insertable, Queryable,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression)]
#[diesel(sql_type = Unsigned<TinyInt>)]
pub enum AccountType {
    Student = 0,
    Professor = 1,
    Administrator = 2,
}

impl<Integer, DB> FromSql<Integer, DB> for AccountType
where
    DB: Backend,
    u8: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        match u8::from_sql(bytes)? {
            0 => Ok(AccountType::Student),
            1 => Ok(AccountType::Professor),
            2 => Ok(AccountType::Administrator),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

#[derive(Debug, Queryable)]
#[allow(unused)]
pub struct User {
    pub id: u32,
    pub password: String,
    pub email: String,
    pub account_type: AccountType,
    pub password_reset_required: bool,
    pub username: Option<String>,
    pub last_login_time: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    password: &'a str,
    email: &'a str,
    account_type: AccountType,
    password_reset_required: bool,
    username: Option<&'a str>,
    last_login_time: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Queryable, Insertable)]
#[diesel(table_name = sessions)]
#[allow(unused)]
pub struct Session {
    pub session_key: Vec<u8>,
    pub user_id: u32,
    pub created_on: NaiveDateTime,
    pub last_refreshed: NaiveDateTime,
    pub timeout_duration_seconds: u32,
}