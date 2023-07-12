use diesel::query_dsl::RunQueryDsl;
use diesel::{ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, Selectable};
use serde::Serialize;

use crate::schema::indicies;
use crate::user::UserId;
use crate::{
    database::Connection,
    error::Error,
    schema::{generations, programs},
};

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

    pub fn get_by_year(connection: &mut Connection, year: u32) -> Result<Generation, Error> {
        generations::table
            .filter(generations::year.eq(year))
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

#[derive(Serialize, Debug)]
pub struct Generations(pub Vec<Generation>);

impl Generations {
    pub fn get(connection: &mut Connection) -> Result<Generations, Error> {
        generations::table
            .get_results(connection)
            .map_err(Error::from)
            .map(|g| Generations { 0: g })
    }
}

#[derive(Queryable, Selectable, Debug, Serialize, Identifiable)]
pub struct Program {
    pub id: u32,
    pub short_name: String,
    pub full_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = programs)]
struct NewProgram {
    short_name: String,
    full_name: String,
}

impl Program {
    pub fn create(
        connection: &mut Connection,
        short_name: String,
        full_name: String,
    ) -> Result<(), Error> {
        diesel::insert_into(programs::table)
            .values(NewProgram {
                short_name,
                full_name,
            })
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn get_by_id(connection: &mut Connection, id: u32) -> Result<Program, Error> {
        programs::table
            .filter(programs::id.eq(id))
            .limit(1)
            .first(connection)
            .map_err(Error::from)
    }

    pub fn get_by_short_name<'a>(
        connection: &mut Connection,
        short_name: &'a str,
    ) -> Result<Program, Error> {
        programs::table
            .filter(programs::short_name.eq(short_name))
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

#[derive(Serialize, Debug)]
pub struct Programs(pub Vec<Program>);

impl Programs {
    pub fn get(connection: &mut Connection) -> Result<Programs, Error> {
        programs::table
            .get_results(connection)
            .map_err(Error::from)
            .map(|g| Programs { 0: g })
    }
}

pub type IndexNumber = u32;

pub struct Index {
    pub id: u32,
    pub program: u32,
    pub generation: u32,
    pub number: IndexNumber,
    pub student: UserId,
}

#[derive(Insertable)]
#[diesel(table_name = indicies)]
pub struct NewIndex {
    program: u32,
    generation: u32,
    number: IndexNumber,
    student: UserId,
}

impl Index {
    pub fn create(
        connection: &mut Connection,
        program: u32,
        generation: u32,
        number: IndexNumber,
        student: UserId,
    ) -> Result<(), Error> {
        diesel::insert_into(indicies::table)
            .values(NewIndex {
                program,
                generation,
                number,
                student,
            })
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }
}
