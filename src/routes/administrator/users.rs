pub mod create;
pub mod delete;
pub mod edit;

use rocket::{form::Form, get, http::Status, post, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    components::users,
    database::Database,
    error::Error,
    localization::Script,
    user::{Administrator, User},
};

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    users: users::LayoutContext,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        users: users::LayoutContext,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            users,
        })
    }
}

#[get("/users", rank = 0)]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
) -> Result<Template, Status> {
    let user = administrator.0;

    let users_context = users::LayoutContext::new(database, None).await?;

    let context = LayoutContext::new(language, user, users_context).await?;

    Ok(Template::render("routes/administrator/users", context))
}

#[derive(Serialize, FromForm, Debug)]
pub struct FormData {
    users_form: users::FormData,
}

#[post("/users", data = "<form>", rank = 0)]
pub async fn post(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    form: Form<FormData>,
) -> Result<Template, Status> {
    let user = administrator.0;

    let users_context =
        users::LayoutContext::new(database, Some(form.into_inner().users_form)).await?;
    let context = LayoutContext::new(language, user, users_context).await?;

    Ok(Template::render("routes/administrator/users", context))
}
