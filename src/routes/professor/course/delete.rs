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

#[derive(Clone, Debug, Serialize)]
struct CourseInfo {
    pub name: String,
}

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    deleting_course: CourseInfo,
    show_you_can_only_delete_own_courses: bool,
    show_success: bool,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        deleting_user: CourseInfo,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            deleting_course: deleting_user,
            show_you_can_only_delete_own_courses: false,
            show_success: false,
        })
    }

    pub fn you_can_only_delete_own_courses(mut self) -> Self {
        self.show_you_can_only_delete_own_courses = true;
        self
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

    let deleting_course_info = CourseInfo {
        name: deleting_course.name,
    };

    let user = professor.0;
    let context = LayoutContext::new(language, user, deleting_course_info).await?;

    if deleting_course.professor != user.id() {
        return Ok(Template::render(
            "routes/professor/course/delete",
            context.you_can_only_delete_own_courses(),
        ));
    }

    Ok(Template::render("routes/professor/course/delete", context))
}

#[post("/course/<url>/delete", rank = 0)]
pub async fn post(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    url: String,
) -> Result<Template, Status> {
    let mut deleting_course = database.run(move |c| Course::get_by_url(c, &url)).await?;

    let deleting_course_info = CourseInfo {
        name: deleting_course.name.clone(),
    };

    let user = professor.0;
    let context = LayoutContext::new(language, user, deleting_course_info).await?;

    if deleting_course.professor != user.id() {
        return Ok(Template::render(
            "routes/professor/course/delete",
            context.you_can_only_delete_own_courses(),
        ));
    }

    deleting_course.update_deleted(true);
    database.run(move |c| deleting_course.store(c)).await?;

    Ok(Template::render(
        "routes/professor/course/delete",
        context.success(),
    ))
}
