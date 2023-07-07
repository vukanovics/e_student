use crate::{
    query_current,
    schema::{grade_assignments_progress, point_assignments_progress},
    user::UserId,
};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
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
    pub created: NaiveDateTime,
    pub course: u32,
    pub name: String,
    pub deleted: bool,
}

#[derive(Serialize, Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = point_assignments)]
pub struct PointAssignment {
    pub id: u32,
    pub created: NaiveDateTime,
    pub course: u32,
    pub name: String,
    pub max_points: u32,
    pub deleted: bool,
}

#[derive(Debug, Serialize)]
pub enum Assignment {
    Grade(GradeAssignment),
    Point(PointAssignment),
}

pub struct Assignments(pub Vec<Assignment>);

query_current!(
    PointAssignment,
    point_assignments,
    PointAssignmentsAlias,
    point_assignments_alias
);

query_current!(
    GradeAssignment,
    grade_assignments,
    GradeAssignmentsAlias,
    grade_assignments_alias
);

impl Assignments {
    pub fn get(connection: &mut Connection, course: CourseId) -> Result<Assignments, Error> {
        let mut point_assignments = PointAssignment::query_current()
            .filter(point_assignments::course.eq(course))
            .load::<PointAssignment>(connection)
            .map(|a| a.into_iter().map(Assignment::Point).collect())
            .map_err(Error::from)?;

        GradeAssignment::query_current()
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
        let mut point_assignments: Vec<GradedAssignment> = PointAssignment::query_current()
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

        GradeAssignment::query_current()
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
