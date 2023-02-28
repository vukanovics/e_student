use rocket::get;

use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn get() -> Template {
    Template::render("routes/index", context! {})
}
