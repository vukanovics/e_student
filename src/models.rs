use std::convert::TryFrom;

use crate::{error::Error, schema::sessions};
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    serialize::ToSql,
    sql_types::{TinyInt, Unsigned},
    AsExpression, FromSqlRow, Insertable, Queryable,
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

#[derive(Clone, Debug, Queryable, Insertable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub session_key: Vec<u8>,
    pub user: u32,
    pub created_on: NaiveDateTime,
    pub last_refreshed: NaiveDateTime,
    pub timeout_duration_seconds: u32,
}
