use rocket::{form::Form, get, http::Status, post, response::Redirect, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    localization::Script,
    models::AccountType,
    user::{Administrator, User},
};

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    editing_user: User,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User, editing_user: User) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            editing_user,
        })
    }
}

#[get("/users/edit/<id>", rank = 0)]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Template, Status> {
    let editing_user = database.run(move |c| User::get_by_id(c, id)).await?;

    let user = administrator.0;
    let context = LayoutContext::new(language, user, editing_user).await?;
    Ok(Template::render("routes/administrator/users/edit", context))
}

#[derive(FromForm)]
pub struct FormData {
    pub username: Option<String>,
    pub email: String,
    pub account_type: u8,
}

#[post("/users/edit/<id>", data = "<form>", rank = 0)]
pub async fn post(
    _administrator: Administrator<'_>,
    database: Database,
    form: Form<FormData>,
    id: u32,
) -> Result<Redirect, Status> {
    let mut editing_user = database.run(move |c| User::get_by_id(c, id)).await?;

    let account_type = AccountType::try_from(form.account_type)?;

    editing_user.update_email(&form.email);
    editing_user.update_account_type(account_type);

    database
        .run(move |c| editing_user.update_database(c))
        .await?;

    Ok(Redirect::to("/users"))
}
