use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    assignment::{GradedAssignment, GradedAssignments},
    base_layout_context::BaseLayoutContext,
    course::Course,
    database::Database,
    error::Error,
    localization::Script,
    user::User,
};

#[derive(Serialize, Debug)]
struct CourseWithAssignments {
    #[serde(flatten)]
    course: Course,
    assignments: Vec<GradedAssignment>,
}

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    course: CourseWithAssignments,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        course: CourseWithAssignments,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            course,
        })
    }
}

#[get("/course/<course>", rank = 0)]
pub async fn get(
    language: Script,
    user: &User,
    database: Database,
    course: String,
) -> Result<Template, Status> {
    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    let user_id = user.id;

    let assignments = database
        .run(move |c| GradedAssignments::get(c, course.id, user_id))
        .await?
        .0;

    let course = CourseWithAssignments {
        course,
        assignments,
    };

    let context = LayoutContext::new(language, user, course).await?;

    Ok(Template::render("routes/student/course", context))
}
