use rocket::{catch, response::Redirect, Request, Responder};
use rocket_dyn_templates::{context, Template};

use crate::user::User;

#[derive(Responder)]
pub enum TemplateOrRedirect {
    Template(Template),
    Redirect(Redirect),
}

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> TemplateOrRedirect {
    if req.guard::<&User>().await.is_success() {
        TemplateOrRedirect::Template(Template::render("catchers/not_found", context!()))
    } else {
        TemplateOrRedirect::Redirect(Redirect::to("/login"))
    }
}
