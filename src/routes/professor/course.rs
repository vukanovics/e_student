use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext, database::Database, error::Error,
    localization::Language, models::Assignment, user::Professor, user::User,
};

#[derive(Clone, Serialize, Debug)]
struct PointAssignmentInfo {
    name: String,
    max_points: u32,
}

#[derive(Clone, Serialize, Debug)]
struct GradeAssignmentInfo {
    name: String,
}

#[derive(Clone, Serialize, Debug)]
enum AssignmentInfo {
    Point(PointAssignmentInfo),
    Grade(GradeAssignmentInfo),
}

impl AssignmentInfo {
    pub fn from_assignment(assignment: Assignment) -> AssignmentInfo {
        match assignment {
            Assignment::Grade(assignment) => AssignmentInfo::Grade(GradeAssignmentInfo {
                name: assignment.name,
            }),
            Assignment::Point(assignment) => AssignmentInfo::Point(PointAssignmentInfo {
                name: assignment.name,
                max_points: assignment.max_points,
            }),
        }
    }
}

#[derive(Clone, Serialize, Debug)]
struct CourseInfo {
    name: String,
    url: String,
    assignments: Vec<AssignmentInfo>,
}

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    course: CourseInfo,
}

impl LayoutContext {
    pub async fn new(
        language: Language,
        user: Option<&User>,
        course: CourseInfo,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            course,
        })
    }
}

#[get("/course/<url>", rank = 1)]
pub async fn get(
    language: Language,
    professor: Professor<'_>,
    database: Database,
    url: String,
) -> Result<Template, Status> {
    let user = professor.0;

    let course = database
        .run(move |c| Database::get_course_by_url(c, &url))
        .await?;

    let assignments = database
        .run(move |c| Database::get_assignments_for_course(c, course.id))
        .await?
        .drain(..)
        .map(AssignmentInfo::from_assignment)
        .collect();

    let course = CourseInfo {
        name: course.name,
        url: course.url,
        assignments,
    };

    let context = LayoutContext::new(language, Some(user), course).await?;

    Ok(Template::render("routes/professor/course", context))
}
