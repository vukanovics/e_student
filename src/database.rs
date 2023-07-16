use crate::error::Error;
use crate::models::Session;
use rocket::FromFormField;
use rocket_sync_db_pools::diesel::prelude::*;
use serde::Serialize;

pub type Connection = diesel::MysqlConnection;

#[rocket_sync_db_pools::database("main_database")]
pub struct Database(Connection);

impl Database {
    pub fn insert_session(
        connection: &mut diesel::MysqlConnection,
        session: &Session,
    ) -> Result<(), Error> {
        use crate::schema::sessions::dsl::sessions;
        diesel::insert_into(sessions)
            .values(session)
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn get_session_by_key(
        connection: &mut diesel::MysqlConnection,
        by_key: Vec<u8>,
    ) -> Result<Session, Error> {
        use crate::schema::sessions::dsl::{session_key, sessions};
        sessions
            .filter(session_key.eq(by_key))
            .limit(1)
            .first::<Session>(connection)
            .map_err(Error::from)
    }
}

#[derive(Debug, FromFormField, Serialize, Clone)]
pub enum SortDirection {
    Ascending,
    Descending,
}
