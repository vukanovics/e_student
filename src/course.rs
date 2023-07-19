use diesel::prelude::*;
use serde::Serialize;

use crate::{
    database::Connection,
    discussion::Discussion,
    error::Error,
    schema::{courses, enrolments},
    user::{User, UserId},
};

#[derive(Clone, Debug, Queryable, Selectable, Serialize)]
pub struct Enrolment {
    pub course: u32,
    pub student: u32,
}

impl Enrolment {
    pub fn create(
        connection: &mut Connection,
        course: CourseId,
        student: UserId,
    ) -> Result<(), Error> {
        diesel::insert_into(enrolments::table)
            .values((
                enrolments::course.eq(course),
                enrolments::student.eq(student),
            ))
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }

    pub fn delete(&self, connection: &mut Connection) -> Result<(), Error> {
        diesel::delete(
            enrolments::table.filter(
                enrolments::course
                    .eq(self.course)
                    .and(enrolments::student.eq(self.student)),
            ),
        )
        .execute(connection)
        .map_err(Error::from)
        .map(|_| ())
    }

    pub fn get(
        connection: &mut Connection,
        course: CourseId,
        student: UserId,
    ) -> Result<Self, Error> {
        enrolments::table
            .filter(
                enrolments::course
                    .eq(course)
                    .and(enrolments::student.eq(student)),
            )
            .limit(1)
            .first(connection)
            .map_err(Error::from)
    }
}

pub type CourseId = u32;

#[derive(Clone, Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: CourseId,
    pub year: u32,
    pub name: String,
    pub url: String,
    pub professor: u32,
    pub discussion: u32,
    pub deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = courses)]
pub struct NewCourse<'a> {
    pub year: u32,
    pub name: &'a str,
    pub url: &'a str,
    pub professor: u32,
    pub discussion: u32,
}

#[derive(Debug)]
pub struct Courses(pub Vec<Course>);

impl Courses {
    pub fn get_all(connection: &mut Connection) -> Result<Self, Error> {
        courses::table
            .filter(courses::deleted.eq(false))
            .load::<Course>(connection)
            .map_err(Error::from)
            .map(|c| Courses { 0: c })
    }

    pub fn get_enrolled(connection: &mut Connection, student: UserId) -> Result<Self, Error> {
        courses::table
            .inner_join(enrolments::table.on(enrolments::course.eq(courses::id)))
            .filter(courses::deleted.eq(false))
            .filter(enrolments::student.eq(student))
            .select(Course::as_select())
            .load::<Course>(connection)
            .map_err(Error::from)
            .map(|c| Courses { 0: c })
    }

    pub fn get_teaching(connection: &mut Connection, professor: UserId) -> Result<Self, Error> {
        courses::table
            .filter(courses::deleted.eq(false))
            .filter(courses::professor.eq(professor))
            .load::<Course>(connection)
            .map_err(Error::from)
            .map(|c| Courses { 0: c })
    }
}

impl Course {
    pub fn create<'a>(
        connection: &mut Connection,
        year: u32,
        name: &'a str,
        url: &'a str,
        professor: UserId,
    ) -> Result<(), Error> {
        let discussion = Discussion::create(connection)?;

        diesel::insert_into(courses::table)
            .values(NewCourse {
                year,
                name,
                url,
                professor,
                discussion,
            })
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn get_by_url<'a>(connection: &mut Connection, url: &'a str) -> Result<Self, Error> {
        courses::table
            .filter(courses::url.eq(url))
            .filter(courses::deleted.eq(false))
            .first::<Course>(connection)
            .map_err(Error::from)
    }

    pub fn delete(&self, connection: &mut Connection) -> Result<(), Error> {
        diesel::update(self)
            .set(courses::deleted.eq(true))
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn authorized_to_edit(&self, user: &User) -> bool {
        user.is_administrator() || self.professor == user.id()
    }
}
