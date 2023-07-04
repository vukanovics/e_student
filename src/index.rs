use diesel::query_dsl::RunQueryDsl;
use diesel::{ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, Selectable};
use serde::Serialize;

use crate::{database::Connection, error::Error, schema::generations};

#[derive(Queryable, Selectable, Debug, Serialize, Identifiable)]
pub struct Generation {
    pub id: u32,
    pub year: u32,
}

#[derive(Insertable)]
#[diesel(table_name = generations)]
struct NewGeneration {
    year: u32,
}

impl Generation {
    pub fn create(connection: &mut Connection, year: u32) -> Result<(), Error> {
        diesel::insert_into(generations::table)
            .values(NewGeneration { year })
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn get_by_id(connection: &mut Connection, id: u32) -> Result<Generation, Error> {
        generations::table
            .filter(generations::id.eq(id))
            .limit(1)
            .first(connection)
            .map_err(Error::from)
    }

    pub fn delete(&self, connection: &mut Connection) -> Result<(), Error> {
        diesel::delete(self)
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }
}

pub struct Generations(pub Vec<Generation>);

impl Generations {
    pub fn get(connection: &mut Connection) -> Result<Generations, Error> {
        generations::table
            .get_results(connection)
            .map_err(Error::from)
            .map(|g| Generations { 0: g })
    }
}
