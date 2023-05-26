use rocket::{
    get,
    http::{CookieJar, Status},
};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext, database::Database, error::Error,
    localization::Language, models::GradedAssignment, user::User,
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
    pub fn from_assignment(assignment: GradedAssignment) -> AssignmentShortInfo {
        match assignment {
            GradedAssignment::Grade((assignment, grade)) => {
                AssignmentShortInfo::Grade(GradeAssignmentShortInfo {
                    name: assignment.name,
                    grade: grade.unwrap_or_default(),
                })
            }
            GradedAssignment::Point((assignment, points)) => {
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
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    courses: Vec<CourseShortInfo>,
}

impl LayoutContext {
    pub async fn new(
        language: Language,
        user: Option<&User>,
        courses: Vec<CourseShortInfo>,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            courses,
        })
    }
}

#[get("/courses", rank = 2)]
pub async fn get(
    language: Language,
    user: &User,
    database: Database,
    _jar: &CookieJar<'_>,
) -> Result<Template, Status> {
    let user_id = user.id();

    let enrolled_courses = database
        .run(move |c| Database::get_courses_for_student(c, user_id))
        .await?;

    let mut courses = Vec::new();

    for course in enrolled_courses {
        let assignments = database
            .run(move |c| Database::get_assignments_for_course_for_user(c, course.id, user_id))
            .await?
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

    let context = LayoutContext::new(language, Some(user), courses.clone()).await?;

    Ok(Template::render("routes/student/courses", context))
}