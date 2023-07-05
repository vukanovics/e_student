pub mod create;

use rocket::{
    get,
    http::{CookieJar, Status},
};
use rocket_dyn_templates::Template;

use crate::{
    course::Courses, database::Database, localization::Script, routes::courses, user::Professor,
};

#[get("/courses", rank = 1)]
pub async fn get(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    _jar: &CookieJar<'_>,
) -> Result<Template, Status> {
    let user = professor.0;
    let user_id = user.id();

    let courses = database
        .run(move |c| Courses::get_teaching(c, user_id))
        .await?
        .0;

    let context = courses::LayoutContext::new(language, user, courses).await?;

    Ok(Template::render("routes/professor/courses", context))
}
