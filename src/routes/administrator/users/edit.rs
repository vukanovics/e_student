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
    let editing_user = database.run(move |c| User::get_by_id(c, id)).await?;

    let account_type = AccountType::try_from(form.account_type)?;

    let current_id = editing_user.id();
    let current_email = editing_user.email();
    let current_account_type = editing_user.account_type();

    if let Some(old_username) = editing_user.username() {
        if let Some(new_username) = form.username.clone() {
            if old_username != new_username {
                database
                    .run(move |c| Database::update_user_username(c, current_id, &new_username))
                    .await?;
            }
        }
    }

    let new_email = form.email.clone();
    if current_email != form.email {
        database
            .run(move |c| Database::update_user_email(c, current_id, &new_email))
            .await?;
    }

    if current_account_type != account_type {
        database
            .run(move |c| Database::update_user_account_type(c, current_id, account_type))
            .await?;
    }

    Ok(Redirect::to("/users"))
}
