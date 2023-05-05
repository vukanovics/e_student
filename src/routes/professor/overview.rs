use rocket::http::{CookieJar, Status};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext, database::Database, error::Error, models::User,
};

#[derive(Clone, Serialize, Debug)]
struct CourseShortInfo {
    name: String,
    url: String,
}

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    courses: Vec<CourseShortInfo>,
}

impl LayoutContext {
    pub async fn new(user: Option<User>, courses: Vec<CourseShortInfo>) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(user).await?,
            courses,
        })
    }
}

pub async fn get(user: User, database: Database, _jar: &CookieJar<'_>) -> Result<Template, Status> {
    let user_id = user.id;

    let enrolled_courses = database
        .run(move |c| Database::get_courses_for_professor(c, user_id))
        .await?;

    let mut courses = Vec::new();

    for course in enrolled_courses {
        let short_info = CourseShortInfo {
            name: course.name,
            url: course.url,
        };

        courses.push(short_info);
    }

    let context = LayoutContext::new(Some(user.clone()), courses.clone()).await?;

    Ok(Template::render("routes/professor/overview", context))
}
