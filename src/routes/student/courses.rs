use rocket::{
    get,
    http::{CookieJar, Status},
};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    assignment::{GradedAssignment, GradedAssignments},
    base_layout_context::BaseLayoutContext,
    course::{Course, Courses},
    database::Database,
    error::Error,
    localization::Script,
    user::User,
};

#[derive(Serialize, Debug)]
pub struct CourseWithAssignments {
    #[serde(flatten)]
    pub course: Course,
    pub assignments: Vec<GradedAssignment>,
}

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    courses: Vec<CourseWithAssignments>,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        courses: Vec<CourseWithAssignments>,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            courses,
        })
    }
}

#[get("/courses", rank = 2)]
pub async fn get(
    language: Script,
    user: &User,
    database: Database,
    _jar: &CookieJar<'_>,
) -> Result<Template, Status> {
    let user_id = user.id();

    let mut enrolled_courses = database
        .run(move |c| Courses::get_enrolled(c, user_id))
        .await?
        .0;

    let mut courses = Vec::new();

    for course in enrolled_courses.drain(..) {
        let course_id = course.id;
        let assignments = database
            .run(move |c| GradedAssignments::get(c, course_id, user_id))
            .await?
            .0;

        courses.push(CourseWithAssignments {
            course,
            assignments,
        });
    }

    let context = LayoutContext::new(language, user, courses).await?;

    Ok(Template::render("routes/student/courses", context))
}
