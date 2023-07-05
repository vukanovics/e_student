use rocket::{get, http::Status, post};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    course::Course,
    database::Database,
    error::Error,
    localization::Script,
    user::{Professor, User},
};

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    deleting_course: Course,
    show_success: bool,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        deleting_course: Course,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            deleting_course,
            show_success: false,
        })
    }

    pub fn success(mut self) -> Self {
        self.show_success = true;
        self
    }
}

#[get("/course/<url>/delete", rank = 0)]
pub async fn get(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    url: String,
) -> Result<Template, Status> {
    let deleting_course = database.run(move |c| Course::get_by_url(c, &url)).await?;

    let user = professor.0;
    let context = LayoutContext::new(language, user, deleting_course.clone()).await?;

    match deleting_course.authorized_to_edit(&user) {
        true => Ok(Template::render("routes/professor/course/delete", context)),
        false => Err(Status::Unauthorized),
    }
}

#[post("/course/<url>/delete", rank = 0)]
pub async fn post(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    url: String,
) -> Result<Template, Status> {
    let mut deleting_course = database.run(move |c| Course::get_by_url(c, &url)).await?;

    let user = professor.0;
    let context = LayoutContext::new(language, user, deleting_course.clone()).await?;

    match deleting_course.authorized_to_edit(&user) {
        true => {
            deleting_course.update_deleted(true);
            database.run(move |c| deleting_course.store(c)).await?;

            Ok(Template::render(
                "routes/professor/course/delete",
                context.success(),
            ))
        }
        false => Err(Status::Unauthorized),
    }
}
