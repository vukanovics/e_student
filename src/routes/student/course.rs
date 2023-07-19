use diesel::Connection;
use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    assignment::{GradedAssignment, GradedAssignments},
    base_layout_context::BaseLayoutContext,
    course::Course,
    database::Database,
    discussion::DiscussionWithComments,
    error::Error,
    localization::Script,
    user::User,
};

#[derive(Serialize, Debug)]
struct CourseWithAssignments {
    #[serde(flatten)]
    course: Course,
    #[serde(flatten)]
    discussion: DiscussionWithComments,
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

#[get("/course/<course>", rank = 2)]
pub async fn get(
    language: Script,
    user: &User,
    database: Database,
    course: String,
) -> Result<Template, Status> {
    let user_id = user.id;
    let course = database
        .run(move |c| {
            c.transaction(move |c| {
                let course = Course::get_by_url(c, &course)?;
                let assignments = GradedAssignments::get(c, course.id, user_id)?.0;
                let discussion = DiscussionWithComments::get(c, course.discussion)?;

                Ok::<_, Error>(CourseWithAssignments {
                    course,
                    discussion,
                    assignments,
                })
            })
        })
        .await?;

    let context = LayoutContext::new(language, user, course).await?;

    Ok(Template::render("routes/student/course", context))
}
