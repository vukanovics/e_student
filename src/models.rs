use std::convert::TryFrom;

use crate::{
    schema::{courses, grade_assignments, point_assignments, sessions, users}, error::Error,
};
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    serialize::ToSql,
    sql_types::{TinyInt, Unsigned},
    AsExpression, FromSqlRow, Insertable, Queryable, Selectable,
};
use serde::Serialize;

#[repr(u8)]
#[derive(AsExpression, FromSqlRow, Serialize, PartialEq, Debug, Clone, Copy)]
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
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
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
            x => Err(Error::InvalidAccountTypeValue),
        }
    }
}

#[derive(Clone, Debug, Queryable)]
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
    pub password: &'a str,
    pub email: &'a str,
    pub account_type: AccountType,
    pub password_reset_required: bool,
    pub username: Option<&'a str>,
    pub last_login_time: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Queryable, Insertable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub session_key: Vec<u8>,
    pub user_id: u32,
    pub created_on: NaiveDateTime,
    pub last_refreshed: NaiveDateTime,
    pub timeout_duration_seconds: u32,
}

#[derive(Clone, Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: u32,
    pub year: u32,
    pub name: String,
    pub url: String,
    pub professor: u32,
}

#[derive(Clone, Debug, Queryable, Insertable)]
#[diesel(table_name = grade_assignments)]
pub struct GradeAssignment {
    pub id: u32,
    pub course: u32,
    pub name: String,
}

#[derive(Clone, Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = point_assignments)]
pub struct PointAssignment {
    pub id: u32,
    pub course: u32,
    pub name: String,
    pub max_points: u32,
}

pub enum Assignment {
    Grade((GradeAssignment, Option<f32>)),
    Point((PointAssignment, Option<u32>)),
}
