use crate::routes::courses;
use rocket::{
    get,
    http::{CookieJar, Status},
};
use rocket_dyn_templates::Template;

use crate::{course::Courses, database::Database, localization::Script, user::Administrator};

#[get("/courses", rank = 0)]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    _jar: &CookieJar<'_>,
) -> Result<Template, Status> {
    let user = administrator.0;

    let courses = database.run(move |c| Courses::get_all(c)).await?.0;

    let context = courses::LayoutContext::new(language, user, courses).await?;

    Ok(Template::render("routes/administrator/courses", context))
}
