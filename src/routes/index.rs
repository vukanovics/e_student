use rocket::{get, http::Status};

use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn get() -> Result<Template, Status> {
    Ok(Template::render("routes/index", context! {}))
}
