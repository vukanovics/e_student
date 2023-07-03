use rocket::{form::Form, get, http::Status, post, response::Redirect, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    localization::Script,
    user::{AccountType, Administrator, User},
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
    pub first_name: String,
    pub last_name: String,
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
    let editing_user = database.run(move |c| User::get_by_id(c, id)).await?;
    let email = form.email.clone();

    database
        .run(move |c| editing_user.update_email(c, &email))
        .await?;

    let editing_user = database.run(move |c| User::get_by_id(c, id)).await?;
    let account_type = AccountType::try_from(form.account_type)?;

    database
        .run(move |c| editing_user.update_account_type(c, account_type))
        .await?;

    let editing_user = database.run(move |c| User::get_by_id(c, id)).await?;
    let first_name = if form.first_name.is_empty() {
        None
    } else {
        Some(form.first_name.clone())
    };

    database
        .run(move |c| editing_user.update_first_name(c, first_name.as_deref()))
        .await?;

    let editing_user = database.run(move |c| User::get_by_id(c, id)).await?;
    let last_name = if form.last_name.is_empty() {
        None
    } else {
        Some(form.last_name.clone())
    };

    database
        .run(move |c| editing_user.update_last_name(c, last_name.as_deref()))
        .await?;

    Ok(Redirect::to("/users"))
}
