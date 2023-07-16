pub mod assignments;
pub mod delete;
pub mod enrol;

use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    assignment::{Assignment, Assignments},
    base_layout_context::BaseLayoutContext,
    course::Course,
    database::Database,
    error::Error,
    localization::Script,
    user::Professor,
    user::User,
};

#[derive(Serialize, Debug)]
struct CourseWithAssignments {
    #[serde(flatten)]
    course: Course,
    assignments: Vec<Assignment>,
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

#[get("/course/<course>", rank = 1)]
pub async fn get(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    course: String,
) -> Result<Template, Status> {
    let user = professor.0;

    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    let assignments = database
        .run(move |c| Assignments::get(c, course.id))
        .await?
        .0;

    let course = CourseWithAssignments {
        course,
        assignments,
    };

    let context = LayoutContext::new(language, user, course).await?;

    Ok(Template::render("routes/professor/course", context))
}
