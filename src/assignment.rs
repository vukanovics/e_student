use crate::{
    schema::{assignments, grade_assignments_progress, point_assignments_progress},
    user::UserId,
};
use diesel::{prelude::*, Insertable, Queryable, Selectable};
use rocket::{FromForm, FromFormField};
use serde::Serialize;

use crate::{
    course::CourseId,
    database::Connection,
    error::Error,
    schema::{grade_assignments, point_assignments},
};

#[derive(Serialize, Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = assignments)]
pub struct BaseAssignment {
    pub id: u32,
    pub course: u32,
    pub name: String,
    pub url: String,
    pub discussion: u32,
    pub deleted: bool,
}

impl BaseAssignment {
    pub fn get(connection: &mut Connection, course: u32, name: &str) -> Result<Self, Error> {
        assignments::table
            .filter(
                assignments::course
                    .eq(course)
                    .and(assignments::name.eq(name)),
            )
            .limit(1)
            .first(connection)
            .map_err(Error::from)
    }

    pub fn create(
        connection: &mut Connection,
        course: u32,
        name: &str,
        url: &str,
    ) -> Result<(), Error> {
        diesel::insert_into(assignments::table)
            .values(&(
                assignments::course.eq(course),
                assignments::name.eq(name),
                assignments::url.eq(url),
            ))
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = grade_assignments)]
pub struct GradeAssignmentData {
    pub id: u32,
    pub assignment: u32,
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = grade_assignments)]
pub struct GradeAssignment {
    #[serde(flatten)]
    #[diesel(embed)]
    pub data: GradeAssignmentData,
    #[serde(flatten)]
    #[diesel(embed)]
    pub base: BaseAssignment,
}

impl GradeAssignment {
    pub fn get(
        connection: &mut Connection,
        course: CourseId,
        url: &str,
    ) -> Result<GradeAssignment, Error> {
        grade_assignments::table
            .inner_join(assignments::table)
            .filter(assignments::course.eq(course).and(assignments::url.eq(url)))
            .limit(1)
            .first(connection)
            .map_err(Error::from)
    }

    pub fn create(
        connection: &mut Connection,
        course: CourseId,
        name: &str,
        url: &str,
    ) -> Result<(), Error> {
        diesel::Connection::transaction(connection, |connection| {
            BaseAssignment::create(connection, course, name, url)?;
            let assignment = BaseAssignment::get(connection, course, name)?;
            diesel::insert_into(grade_assignments::table)
                .values(grade_assignments::assignment.eq(assignment.id))
                .execute(connection)
                .map_err(Error::from)
        })
        .map(|_| ())
    }

    pub fn grade(
        connection: &mut Connection,
        grade_assignment_id: u32,
        student: UserId,
        grade: GradeAssignmentGrade,
    ) -> Result<(), Error> {
        #[derive(Selectable, Queryable, Identifiable)]
        #[diesel(table_name = grade_assignments_progress)]
        #[diesel(primary_key(assignment, student))]
        struct Grade {
            assignment: u32,
            student: u32,
            #[diesel(column_name = "grade_major")]
            _grade_major: u8,
            #[diesel(column_name = "grade_minor")]
            _grade_minor: u8,
        }
        let previous_grade = grade_assignments_progress::table
            .filter(
                grade_assignments_progress::assignment
                    .eq(grade_assignment_id)
                    .and(grade_assignments_progress::student.eq(student)),
            )
            .limit(1)
            .first::<Grade>(connection)
            .map_err(Error::from);
        match previous_grade {
            Ok(previous_grade) => diesel::update(&previous_grade)
                .set((
                    grade_assignments_progress::grade_minor.eq(grade.minor),
                    grade_assignments_progress::grade_major.eq(grade.major),
                ))
                .execute(connection)
                .map_err(Error::from)
                .map(|_| ()),
            Err(Error::DatabaseEntryNotFound) => {
                diesel::insert_into(grade_assignments_progress::table)
                    .values((
                        grade_assignments_progress::assignment.eq(grade_assignment_id),
                        grade_assignments_progress::student.eq(student),
                        grade_assignments_progress::grade_minor.eq(grade.minor),
                        grade_assignments_progress::grade_major.eq(grade.major),
                    ))
                    .execute(connection)
                    .map_err(Error::from)
                    .map(|_| ())
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = point_assignments)]
pub struct PointAssignmentData {
    pub id: u32,
    pub assignment: u32,
    pub max_points: u32,
}

#[derive(Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = point_assignments)]
pub struct PointAssignment {
    #[serde(flatten)]
    #[diesel(embed)]
    pub data: PointAssignmentData,
    #[serde(flatten)]
    #[diesel(embed)]
    pub base: BaseAssignment,
}

impl PointAssignment {
    pub fn get(
        connection: &mut Connection,
        course: CourseId,
        url: &str,
    ) -> Result<PointAssignment, Error> {
        point_assignments::table
            .inner_join(assignments::table)
            .filter(assignments::course.eq(course).and(assignments::url.eq(url)))
            .limit(1)
            .first(connection)
            .map_err(Error::from)
    }

    pub fn create<'a>(
        connection: &mut Connection,
        course: CourseId,
        name: &'a str,
        url: &'a str,
        max_points: u32,
    ) -> Result<(), Error> {
        diesel::Connection::transaction(connection, |connection| {
            BaseAssignment::create(connection, course, name, url)?;
            let assignment = BaseAssignment::get(connection, course, name)?;
            diesel::insert_into(point_assignments::table)
                .values((
                    point_assignments::assignment.eq(assignment.id),
                    point_assignments::max_points.eq(max_points),
                ))
                .execute(connection)
                .map_err(Error::from)
        })
        .map(|_| ())
    }

    pub fn grade(
        connection: &mut Connection,
        points_assignment_id: u32,
        student: UserId,
        points: u32,
    ) -> Result<(), Error> {
        #[derive(Selectable, Queryable, Identifiable)]
        #[diesel(table_name = point_assignments_progress)]
        #[diesel(primary_key(assignment, student))]
        struct Grade {
            assignment: u32,
            student: u32,
            #[diesel(column_name = "points")]
            _points: u32,
        }
        let previous_grade = point_assignments_progress::table
            .filter(
                point_assignments_progress::assignment
                    .eq(points_assignment_id)
                    .and(point_assignments_progress::student.eq(student)),
            )
            .limit(1)
            .first::<Grade>(connection)
            .map_err(Error::from);
        match previous_grade {
            Ok(previous_grade) => diesel::update(&previous_grade)
                .set(point_assignments_progress::points.eq(points))
                .execute(connection)
                .map_err(Error::from)
                .map(|_| ()),
            Err(Error::DatabaseEntryNotFound) => {
                diesel::insert_into(point_assignments_progress::table)
                    .values((
                        point_assignments_progress::assignment.eq(points_assignment_id),
                        point_assignments_progress::student.eq(student),
                        point_assignments_progress::points.eq(points),
                    ))
                    .execute(connection)
                    .map_err(Error::from)
                    .map(|_| ())
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Assignment {
    Grade(GradeAssignment),
    Point(PointAssignment),
}

impl Assignment {
    pub fn get(
        connection: &mut Connection,
        course: CourseId,
        url: &str,
    ) -> Result<Assignment, Error> {
        PointAssignment::get(connection, course, url)
            .map(|p| Assignment::Point(p))
            .or_else(|_| {
                GradeAssignment::get(connection, course, url).map(|g| Assignment::Grade(g))
            })
    }
}
pub struct Assignments(pub Vec<Assignment>);

impl Assignments {
    pub fn get(connection: &mut Connection, course: CourseId) -> Result<Assignments, Error> {
        let mut point_assignments = point_assignments::table
            .inner_join(assignments::table)
            .filter(assignments::course.eq(course))
            .load::<PointAssignment>(connection)
            .map(|a| a.into_iter().map(Assignment::Point).collect())
            .map_err(Error::from)?;

        grade_assignments::table
            .inner_join(assignments::table)
            .filter(assignments::course.eq(course))
            .load::<GradeAssignment>(connection)
            .map(|a| a.into_iter().map(Assignment::Grade).collect())
            .map_err(Error::from)
            .map(|mut a: Vec<Assignment>| {
                a.append(&mut point_assignments);
                Assignments { 0: a }
            })
    }
}

pub const GRADE_MAJOR_MAX: u8 = 10;
pub const GRADE_MINOR_MAX: u8 = 99;

#[derive(Serialize, Debug, Selectable, Queryable, Clone, FromForm, PartialEq, Default)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = grade_assignments_progress)]
pub struct GradeAssignmentGrade {
    #[diesel(column_name = "grade_major")]
    #[field(validate = range(0..=GRADE_MAJOR_MAX as isize))]
    pub major: u8,
    #[diesel(column_name = "grade_minor")]
    #[field(validate = range(0..=GRADE_MINOR_MAX as isize))]
    pub minor: u8,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct GradedGradeAssignment {
    #[serde(flatten)]
    #[diesel(embed)]
    pub assignment: GradeAssignment,
    #[serde(flatten)]
    #[diesel(embed)]
    pub grade: Option<GradeAssignmentGrade>,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct GradedPointAssignment {
    #[serde(flatten)]
    #[diesel(embed)]
    pub assignment: PointAssignment,
    #[diesel(select_expression = point_assignments_progress::points.nullable())]
    #[diesel(select_expression_type = diesel::dsl::Nullable<point_assignments_progress::points>)]
    pub points: Option<u32>,
}

#[derive(Serialize, Debug)]
pub enum GradedAssignment {
    Grade(GradedGradeAssignment),
    Point(GradedPointAssignment),
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
            .inner_join(assignments::table)
            .filter(
                assignments::course
                    .eq(course)
                    .and(assignments::deleted.eq(false)),
            )
            .select(GradedPointAssignment::as_select())
            .load::<GradedPointAssignment>(connection)
            .map(|a| a.into_iter().map(GradedAssignment::Point).collect())
            .map_err(Error::from)?;

        grade_assignments::table
            .inner_join(assignments::table)
            .left_join(
                grade_assignments_progress::table.on(grade_assignments::id
                    .eq(grade_assignments_progress::assignment)
                    .and(grade_assignments_progress::student.eq(student))),
            )
            .filter(
                assignments::course
                    .eq(course)
                    .and(assignments::deleted.eq(false)),
            )
            .select(GradedGradeAssignment::as_select())
            .load::<GradedGradeAssignment>(connection)
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
