use rocket::{get, http::Status, post};

use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub fn get() -> Result<Template, Status> {
    Ok(Template::render("routes/login", context! {}))
}

#[post("/login")]
pub fn post() -> Result<Template, Status> {
    Ok(Template::render("routes/login", context! {}))
}
