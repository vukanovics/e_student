use crate::error::Error;
use crate::models::{
    AccountType, Assignment, Course, GradeAssignment, NewCourse, NewUser, PointAssignment, Session,
    User,
};
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
            .map_err(Error::from)
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
            .map_err(Error::from)
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

    #[allow(unused)]
    pub fn get_course_by_url<'a>(
        connection: &mut diesel::MysqlConnection,
        by_url: &'a str,
    ) -> Result<Course, Error> {
        use crate::schema::courses::dsl::{courses, url};
        courses
            .filter(url.eq(by_url))
            .limit(1)
            .first::<Course>(connection)
            .map_err(Error::from)
    }

    pub fn get_all_courses(connection: &mut diesel::MysqlConnection) -> Result<Vec<Course>, Error> {
        use crate::schema::courses::dsl::courses;
        courses.load::<Course>(connection).map_err(Error::from)
    }

    pub fn get_courses_for_student(
        connection: &mut diesel::MysqlConnection,
        for_student: u32,
    ) -> Result<Vec<Course>, Error> {
        use crate::schema::courses::dsl::{courses, id};
        use crate::schema::enrolments::dsl::{course, enrolments, student};
        courses
            .inner_join(enrolments.on(course.eq(id)))
            .filter(student.eq(for_student))
            .select(Course::as_select())
            .load::<Course>(connection)
            .map_err(Error::from)
    }

    pub fn get_courses_for_professor(
        connection: &mut diesel::MysqlConnection,
        for_professor: u32,
    ) -> Result<Vec<Course>, Error> {
        use crate::schema::courses::dsl::{courses, professor};
        courses
            .filter(professor.eq(for_professor))
            .select(Course::as_select())
            .load::<Course>(connection)
            .map_err(Error::from)
    }

    pub fn get_assignments_for_course_for_user(
        connection: &mut diesel::MysqlConnection,
        for_course: u32,
        for_user: u32,
    ) -> Result<Vec<Assignment>, Error> {
        use crate::schema::point_assignments;
        use crate::schema::point_assignments_progress;

        let mut point_assignments: Vec<Assignment> = point_assignments::table
            .left_join(point_assignments_progress::table)
            .filter(
                point_assignments::course
                    .eq(for_course)
                    .and(point_assignments_progress::student.eq(for_user)),
            )
            .select((
                point_assignments::all_columns,
                point_assignments_progress::points.nullable(),
            ))
            .load::<(PointAssignment, Option<u32>)>(connection)
            .map(|a| a.into_iter().map(Assignment::Point).collect())
            .map_err(Error::from)?;

        use crate::schema::grade_assignments;
        use crate::schema::grade_assignments_progress;

        grade_assignments::table
            .left_join(grade_assignments_progress::table)
            .filter(grade_assignments::course.eq(for_course))
            .select((
                grade_assignments::all_columns,
                grade_assignments_progress::grade.nullable(),
            ))
            .load::<(GradeAssignment, Option<f32>)>(connection)
            .map(|a| a.into_iter().map(Assignment::Grade).collect())
            .map_err(Error::from)
            .map(|mut a: Vec<Assignment>| {
                a.append(&mut point_assignments);
                a
            })
    }

    pub fn get_all_users(connection: &mut diesel::MysqlConnection) -> Result<Vec<User>, Error> {
        use crate::schema::users;
        users::table.load::<User>(connection).map_err(Error::from)
    }

    pub fn delete_user_by_id(
        connection: &mut diesel::MysqlConnection,
        by_id: u32,
    ) -> Result<(), Error> {
        use crate::schema::users::{self, id};
        diesel::delete(users::table.filter(id.eq(by_id)))
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }

    pub fn create_new_user<'a>(
        connection: &mut diesel::MysqlConnection,
        email: &'a str,
        password: &'a str,
        account_type: AccountType,
    ) -> Result<(), Error> {
        use crate::schema::users::dsl::users;
        let new_user = NewUser {
            password,
            email,
            account_type,
            password_reset_required: true,
            username: None,
            last_login_time: None,
        };
        diesel::insert_into(users)
            .values(new_user)
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }

    pub fn update_user_username<'a>(
        connection: &mut diesel::MysqlConnection,
        by_id: u32,
        new_username: &'a str,
    ) -> Result<(), Error> {
        use crate::schema::users::dsl::{id, username, users};
        diesel::update(users)
            .filter(id.eq(by_id))
            .set(username.eq(new_username))
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }

    pub fn update_user_email<'a>(
        connection: &mut diesel::MysqlConnection,
        by_id: u32,
        new_email: &'a str,
    ) -> Result<(), Error> {
        use crate::schema::users::dsl::{email, id, users};
        diesel::update(users)
            .filter(id.eq(by_id))
            .set(email.eq(new_email))
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }

    pub fn update_user_account_type(
        connection: &mut diesel::MysqlConnection,
        by_id: u32,
        new_account_type: AccountType,
    ) -> Result<(), Error> {
        use crate::schema::users::dsl::{account_type, id, users};
        diesel::update(users)
            .filter(id.eq(by_id))
            .set(account_type.eq(new_account_type))
            .execute(connection)
            .map_err(Error::from)
            .map(|_| ())
    }

    pub fn insert_course(
        connection: &mut diesel::MysqlConnection,
        new_course: NewCourse,
    ) -> Result<(), Error> {
        use crate::schema::courses::dsl::courses;
        diesel::insert_into(courses)
            .values(new_course)
            .execute(connection)
            .map(|_| ())
            .map_err(Error::from)
    }
}
