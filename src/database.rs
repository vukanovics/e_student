use crate::error::Error;
use crate::models::Session;
use rocket_sync_db_pools::diesel::prelude::*;

pub type Connection = diesel::MysqlConnection;
pub type Backend = diesel::mysql::Mysql;

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

#[macro_export]
macro_rules! query_current {
    ( $result:ty, $table:ident, $table_alias_type:ident, $table_alias:ident ) => {
        diesel::alias!($table as $table_alias: $table_alias_type);

        impl $result {
            fn query_current() -> diesel::helper_types::Select<
            diesel::helper_types::Filter<
                diesel::helper_types::LeftJoin<
                    $table::table,
                    diesel::helper_types::On<
                        diesel::query_source::Alias<$table_alias_type>,
                        diesel::helper_types::And<
                            diesel::helper_types::Eq<$table::id, diesel::query_source::AliasedField<$table_alias_type, $table::id>>,
                            diesel::helper_types::Lt<$table::created, diesel::query_source::AliasedField<$table_alias_type, $table::created>>,
                        >,
                    >,
                >,
                diesel::helper_types::IsNull<diesel::query_source::AliasedField<$table_alias_type, $table::id>>,
            >,
            diesel::helper_types::AsSelect<$result, crate::database::Backend>,
        > {
                let id2 = $table_alias.field($table::id);
                let created2 = $table_alias.field($table::created);

                // approach from
                // https://medium.com/@hanifaarrumaisha/query-greatest-n-per-group-a1516fd4b0f6

                $table::table
                    .left_outer_join($table_alias.on($table::id.eq(id2).and($table::created.lt(created2))))
                    .filter(id2.is_null())
                    .select(<$result>::as_select())
            }
        }
    }
}
