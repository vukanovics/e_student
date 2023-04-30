use rocket::{
    get,
    http::{CookieJar, Status},
};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    application::get_user_from_jar,
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    models::{AccountType, Assignment, User},
};

#[derive(Clone, Serialize, Debug)]
struct PointAssignmentData {
    name: String,
    points: u32,
    max_points: u32,
}

#[derive(Clone, Serialize, Debug)]
struct GradeAssignmentData {
    name: String,
    grade: f32,
}

#[derive(Clone, Serialize, Debug)]
enum AssignmentData {
    Point(PointAssignmentData),
    Grade(GradeAssignmentData),
}

impl AssignmentData {
    pub fn from_assignment(assignment: Assignment) -> AssignmentData {
        match assignment {
            Assignment::GradeAssignment(assignment) => AssignmentData::Grade(GradeAssignmentData {
                name: assignment.name,
                grade: 0f32,
            }),
            Assignment::PointAssignment(assignment) => AssignmentData::Point(PointAssignmentData {
                name: assignment.name,
                points: 0u32,
                max_points: assignment.max_points,
            }),
        }
    }
}
#[derive(Clone, Serialize, Debug)]
struct StudentCourseLayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    name: String,
    assignments: Vec<AssignmentData>,
}

impl StudentCourseLayoutContext {
    pub async fn new(
        user: Option<User>,
        name: String,
        assignments: Vec<AssignmentData>,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(user).await?,
            name,
            assignments,
        })
    }
}

#[get("/course/<url>")]
pub async fn get(database: Database, jar: &CookieJar<'_>, url: &str) -> Result<Template, Status> {
    let user = get_user_from_jar(&database, jar).await?;
    let user = match user {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let url = url.to_owned();
    let course = database
        .run(move |c| Database::get_course_by_url(c, &url))
        .await?;

    let assignments = database
        .run(move |c| Database::get_assignments_by_course(c, course.id))
        .await?;

    let assignments = assignments
        .into_iter()
        .map(|a| AssignmentData::from_assignment(a))
        .collect();

    match user.account_type {
        AccountType::Student => Ok(Template::render(
            "routes/student/course",
            StudentCourseLayoutContext::new(Some(user), course.name, assignments).await?,
        )),
        AccountType::Professor => Ok(Template::render(
            "routes/professor/course",
            StudentCourseLayoutContext::new(Some(user), course.name, assignments).await?,
        )),
        AccountType::Administrator => Ok(Template::render(
            "routes/administrator/course",
            StudentCourseLayoutContext::new(Some(user), course.name, assignments).await?,
        )),
    }
}
