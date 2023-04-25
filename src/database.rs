use crate::error::Error;
use crate::models::User;
use rocket_sync_db_pools::diesel::prelude::*;

#[rocket_sync_db_pools::database("main_database")]
pub struct Database(diesel::MysqlConnection);

impl Database {
    #[allow(unused)]
    pub fn get_user_by_id(
        connection: &mut diesel::MysqlConnection,
        user_id: u32,
    ) -> Result<User, Error> {
        use crate::schema::users::dsl::{id, users};
        users
            .filter(id.eq(user_id))
            .limit(1)
            .first::<User>(connection)
            .map_err(|e| e.into())
    }

    #[allow(unused)]
    pub fn get_user_by_username_or_email<'a>(
        connection: &mut diesel::MysqlConnection,
        username_or_email: &'a str,
    ) -> Result<User, Error> {
        use crate::schema::users::dsl::{email, username, users};
        users
            .filter(username.eq(username_or_email).or(email.eq(username_or_email)))
            .limit(1)
            .first::<User>(connection)
            .map_err(|e| e.into())
    }
}
