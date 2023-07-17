use crate::{
    schema::{grade_assignments_progress, point_assignments_progress},
    user::UserId,
};
use diesel::{Insertable, Queryable, Selectable};
use rocket::FromFormField;
use serde::Serialize;

use crate::{
    course::CourseId,
    database::Connection,
    error::Error,
    schema::{grade_assignments, point_assignments},
};
use diesel::prelude::*;

#[derive(Serialize, Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = grade_assignments)]
pub struct GradeAssignment {
    pub id: u32,
    pub course: u32,
    pub name: String,
    pub url: String,
    pub deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = grade_assignments)]
pub struct NewGradeAssignment<'a> {
    pub course: CourseId,
    pub name: &'a str,
    pub url: &'a str,
}

impl GradeAssignment {
    pub fn create<'a>(
        connection: &mut Connection,
        course: CourseId,
        name: &'a str,
        url: &'a str,
    ) -> Result<(), Error> {
        diesel::insert_into(grade_assignments::table)
            .values(NewGradeAssignment { course, name, url })
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }
}

#[derive(Serialize, Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = point_assignments)]
pub struct PointAssignment {
    pub id: u32,
    pub course: u32,
    pub name: String,
    pub url: String,
    pub max_points: u32,
    pub deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = point_assignments)]
pub struct NewPointAssignment<'a> {
    pub course: CourseId,
    pub name: &'a str,
    pub url: &'a str,
    pub max_points: u32,
}

impl PointAssignment {
    pub fn create<'a>(
        connection: &mut Connection,
        course: CourseId,
        name: &'a str,
        url: &'a str,
        max_points: u32,
    ) -> Result<(), Error> {
        diesel::insert_into(point_assignments::table)
            .values(NewPointAssignment {
                course,
                name,
                url,
                max_points,
            })
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }
}

#[derive(Debug, Serialize)]
pub enum Assignment {
    Grade(GradeAssignment),
    Point(PointAssignment),
}

pub struct Assignments(pub Vec<Assignment>);

impl Assignments {
    pub fn get(connection: &mut Connection, course: CourseId) -> Result<Assignments, Error> {
        let mut point_assignments = point_assignments::table
            .filter(point_assignments::course.eq(course))
            .load::<PointAssignment>(connection)
            .map(|a| a.into_iter().map(Assignment::Point).collect())
            .map_err(Error::from)?;

        grade_assignments::table
            .filter(grade_assignments::course.eq(course))
            .load::<GradeAssignment>(connection)
            .map(|a| a.into_iter().map(Assignment::Grade).collect())
            .map_err(Error::from)
            .map(|mut a: Vec<Assignment>| {
                a.append(&mut point_assignments);
                Assignments { 0: a }
            })
    }
}

#[derive(Serialize, Debug)]
pub enum GradedAssignment {
    Grade((GradeAssignment, Option<f32>)),
    Point((PointAssignment, Option<u32>)),
}

pub struct GradedAssignments(pub Vec<GradedAssignment>);

impl GradedAssignments {
    pub fn get(
        connection: &mut Connection,
        course: CourseId,
        student: UserId,
    ) -> Result<GradedAssignments, Error> {
        let mut point_assignments: Vec<GradedAssignment> = point_assignments::table
            .left_join(
                point_assignments_progress::table.on(point_assignments::id
                    .eq(point_assignments_progress::assignment)
                    .and(point_assignments_progress::student.eq(student))),
            )
            .filter(
                point_assignments::course
                    .eq(course)
                    .and(point_assignments::deleted.eq(false)),
            )
            .select((
                point_assignments::all_columns,
                point_assignments_progress::points.nullable(),
            ))
            .load::<(PointAssignment, Option<u32>)>(connection)
            .map(|a| a.into_iter().map(GradedAssignment::Point).collect())
            .map_err(Error::from)?;

        grade_assignments::table
            .left_join(
                grade_assignments_progress::table.on(grade_assignments::id
                    .eq(grade_assignments_progress::assignment)
                    .and(grade_assignments_progress::student.eq(student))),
            )
            .filter(
                grade_assignments::course
                    .eq(course)
                    .and(grade_assignments::deleted.eq(false)),
            )
            .select((
                grade_assignments::all_columns,
                grade_assignments_progress::grade.nullable(),
            ))
            .load::<(GradeAssignment, Option<f32>)>(connection)
            .map(|a| a.into_iter().map(GradedAssignment::Grade).collect())
            .map_err(Error::from)
            .map(|mut a: Vec<GradedAssignment>| {
                a.append(&mut point_assignments);
                GradedAssignments { 0: a }
            })
    }
}

#[derive(Serialize, Debug, FromFormField, Clone)]
pub enum AssignmentType {
    Grade,
    Point,
}
