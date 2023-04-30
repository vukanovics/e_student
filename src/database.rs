use crate::error::Error;
use crate::models::{Assignment, Course, GradeAssignment, PointAssignment, Session, User};
use rocket_sync_db_pools::diesel::prelude::*;

#[rocket_sync_db_pools::database("main_database")]
pub struct Database(diesel::MysqlConnection);

impl Database {
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

    pub fn get_user_by_username_or_email<'a>(
        connection: &mut diesel::MysqlConnection,
        username_or_email: &'a str,
    ) -> Result<User, Error> {
        use crate::schema::users::dsl::{email, username, users};
        users
            .filter(
                username
                    .eq(username_or_email)
                    .or(email.eq(username_or_email)),
            )
            .limit(1)
            .first::<User>(connection)
            .map_err(|e| e.into())
    }

    pub fn insert_session(
        connection: &mut diesel::MysqlConnection,
        session: &Session,
    ) -> Result<(), Error> {
        use crate::schema::sessions::dsl::sessions;
        diesel::insert_into(sessions)
            .values(session)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
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
            .map_err(|e| e.into())
    }

    pub fn get_course_by_url<'a>(
        connection: &mut diesel::MysqlConnection,
        by_url: &'a str,
    ) -> Result<Course, Error> {
        use crate::schema::courses::dsl::{courses, url};
        courses
            .filter(url.eq(by_url))
            .limit(1)
            .first::<Course>(connection)
            .map_err(|e| e.into())
    }

    pub fn get_enrolled_courses_by_user_id(
        connection: &mut diesel::MysqlConnection,
        by_user_id: u32,
    ) -> Result<Vec<Course>, Error> {
        use crate::schema::courses::dsl::{courses, id};
        use crate::schema::enrolments::dsl::{course, enrolments, student};
        courses
            .inner_join(enrolments.on(course.eq(id)))
            .filter(student.eq(by_user_id))
            .select(Course::as_select())
            .load::<Course>(connection)
            .map_err(Error::Diesel)
    }

    pub fn get_assignments_by_course(
        connection: &mut diesel::MysqlConnection,
        by_course: u32,
    ) -> Result<Vec<Assignment>, Error> {
        use crate::schema::point_assignments;

        let mut point_assignments = point_assignments::table
            .filter(point_assignments::course.eq(by_course))
            .load::<PointAssignment>(connection)
            .map(|a| a.into_iter().map(Assignment::PointAssignment).collect())
            .map_err(Error::Diesel)?;

        use crate::schema::grade_assignments;

        grade_assignments::table
            .filter(grade_assignments::course.eq(by_course))
            .load::<GradeAssignment>(connection)
            .map(|a| a.into_iter().map(Assignment::GradeAssignment).collect())
            .map_err(Error::Diesel)
            .map(|mut a: Vec<Assignment>| {
                a.append(&mut point_assignments);
                a
            })
    }
}
