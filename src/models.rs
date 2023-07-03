use crate::schema::sessions;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Clone, Debug, Queryable, Insertable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub session_key: Vec<u8>,
    pub user: u32,
    pub created_on: NaiveDateTime,
    pub last_refreshed: NaiveDateTime,
    pub timeout_duration_seconds: u32,
}
