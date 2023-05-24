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
    localization::Language,
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
            Assignment::Grade((assignment, grade)) => AssignmentData::Grade(GradeAssignmentData {
                name: assignment.name,
                grade: grade.unwrap_or_default(),
            }),
            Assignment::Point((assignment, points)) => AssignmentData::Point(PointAssignmentData {
                name: assignment.name,
                points: points.unwrap_or_default(),
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
        language: Language,
        user: Option<User>,
        name: String,
        assignments: Vec<AssignmentData>,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            name,
            assignments,
        })
    }
}

#[get("/<language>/course/<url>")]
pub async fn get(
    database: Database,
    jar: &CookieJar<'_>,
    url: &str,
    language: &str,
) -> Result<Template, Status> {
    let language = Language::from_code(language)?;

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
        .run(move |c| Database::get_assignments_for_course_for_user(c, course.id, user.id))
        .await?;

    let assignments = assignments
        .into_iter()
        .map(|a| AssignmentData::from_assignment(a))
        .collect();

    match user.account_type {
        AccountType::Student => Ok(Template::render(
            "routes/student/course",
            StudentCourseLayoutContext::new(language, Some(user), course.name, assignments).await?,
        )),
        AccountType::Professor => Ok(Template::render(
            "routes/professor/course",
            StudentCourseLayoutContext::new(language, Some(user), course.name, assignments).await?,
        )),
        AccountType::Administrator => Ok(Template::render(
            "routes/administrator/course",
            StudentCourseLayoutContext::new(language, Some(user), course.name, assignments).await?,
        )),
    }
}