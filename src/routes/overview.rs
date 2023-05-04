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
struct PointAssignmentShortInfo {
    name: String,
    points: u32,
    max_points: u32,
}

#[derive(Clone, Serialize, Debug)]
struct GradeAssignmentShortInfo {
    name: String,
    grade: f32,
}

#[derive(Clone, Serialize, Debug)]
enum AssignmentShortInfo {
    Point(PointAssignmentShortInfo),
    Grade(GradeAssignmentShortInfo),
}

impl AssignmentShortInfo {
    pub fn from_assignment(assignment: Assignment) -> AssignmentShortInfo {
        match assignment {
            Assignment::Grade((assignment, grade)) => {
                AssignmentShortInfo::Grade(GradeAssignmentShortInfo {
                    name: assignment.name,
                    grade: grade.unwrap_or_default(),
                })
            }
            Assignment::Point((assignment, points)) => {
                AssignmentShortInfo::Point(PointAssignmentShortInfo {
                    name: assignment.name,
                    points: points.unwrap_or_default(),
                    max_points: assignment.max_points,
                })
            }
        }
    }
}

#[derive(Clone, Serialize, Debug)]
struct CourseShortInfo {
    name: String,
    url: String,
    assignments: Vec<AssignmentShortInfo>,
}

#[derive(Clone, Serialize, Debug)]
struct StudentOverviewLayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    courses: Vec<CourseShortInfo>,
}

impl StudentOverviewLayoutContext {
    pub async fn new(user: Option<User>, courses: Vec<CourseShortInfo>) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(user).await?,
            courses,
        })
    }
}

#[get("/overview")]
pub async fn get(database: Database, jar: &CookieJar<'_>) -> Result<Template, Status> {
    let user = get_user_from_jar(&database, jar).await?;
    let user = match user {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let user_id = user.id;
    let enrolled_courses = database
        .run(move |c| Database::get_enrolled_courses_by_user_id(c, user_id))
        .await?;

    let mut courses = Vec::new();

    for course in enrolled_courses {
        let assignments = database
            .run(move |c| Database::get_assignments_for_course_for_user(c, course.id, user.id))
            .await?;
        let assignments = assignments
            .into_iter()
            .map(|a| AssignmentShortInfo::from_assignment(a))
            .collect();
        let short_info = CourseShortInfo {
            name: course.name,
            url: course.url,
            assignments,
        };
        courses.push(short_info);
    }

    let context = StudentOverviewLayoutContext::new(Some(user.clone()), courses.clone()).await?;
    match user.account_type {
        AccountType::Student => Ok(Template::render("routes/student/overview", context)),
        AccountType::Professor => Ok(Template::render(
            "routes/professor/overview",
            StudentOverviewLayoutContext::new(Some(user), courses).await?,
        )),
        AccountType::Administrator => Ok(Template::render(
            "routes/administrator/overview",
            StudentOverviewLayoutContext::new(Some(user), courses).await?,
        )),
    }
}
