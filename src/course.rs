use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::{
    database::Connection,
    error::Error,
    query_current,
    schema::{courses, enrolments},
    user::{User, UserId},
};

pub type CourseId = u32;

#[derive(Clone, Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: CourseId,
    pub created: NaiveDateTime,
    pub year: u32,
    pub name: String,
    pub url: String,
    pub professor: u32,
    pub deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = courses)]
pub struct UpdateCourse<'a> {
    pub id: CourseId,
    pub year: u32,
    pub name: &'a str,
    pub url: &'a str,
    pub professor: u32,
    pub deleted: bool,
}

impl<'a> From<&'a Course> for UpdateCourse<'a> {
    fn from(value: &'a Course) -> Self {
        UpdateCourse {
            id: value.id,
            year: value.year,
            name: &value.name,
            url: &value.url,
            professor: value.professor,
            deleted: value.deleted,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = courses)]
pub struct NewCourse<'a> {
    pub year: u32,
    pub name: &'a str,
    pub url: &'a str,
    pub professor: u32,
}

#[derive(Debug)]
pub struct Courses(pub Vec<Course>);

impl From<Vec<Course>> for Courses {
    fn from(value: Vec<Course>) -> Self {
        Self { 0: value }
    }
}

query_current!(Course, courses, CoursesAlias, courses_alias);

impl Courses {
    pub fn get_all(connection: &mut Connection) -> Result<Self, Error> {
        Course::query_current()
            .filter(courses::deleted.eq(false))
            .load::<Course>(connection)
            .map_err(Error::from)
            .map(Self::from)
    }

    pub fn get_enrolled(connection: &mut Connection, student: UserId) -> Result<Self, Error> {
        Course::query_current()
            .inner_join(enrolments::table.on(enrolments::course.eq(courses::id)))
            .filter(
                enrolments::student
                    .eq(student)
                    .and(courses::deleted.eq(false)),
            )
            .load::<Course>(connection)
            .map_err(Error::from)
            .map(Self::from)
    }

    pub fn get_teaching(connection: &mut Connection, professor: UserId) -> Result<Self, Error> {
        Course::query_current()
            .filter(
                courses::professor
                    .eq(professor)
                    .and(courses::deleted.eq(false)),
            )
            .load::<Course>(connection)
            .map_err(Error::from)
            .map(Self::from)
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
        diesel::insert_into(courses::table)
            .values(NewCourse {
                year,
                name,
                url,
                professor,
            })
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn get_by_url<'a>(connection: &mut Connection, url: &'a str) -> Result<Self, Error> {
        Course::query_current()
            .filter(courses::url.eq(url))
            .order(courses::created.desc())
            .filter(courses::deleted.eq(false))
            .first::<Course>(connection)
            .map_err(Error::from)
    }

    pub fn update_deleted(&mut self, deleted: bool) -> () {
        self.deleted = deleted;
    }

    pub fn store(&self, connection: &mut Connection) -> Result<(), Error> {
        diesel::insert_into(courses::table)
            .values(UpdateCourse::from(self))
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn authorized_to_edit(&self, user: &User) -> bool {
        user.is_administrator() || self.professor == user.id()
    }
}
