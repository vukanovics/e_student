use rocket::{get, http::Status, post, response::Redirect};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    localization::Script,
    user::{Administrator, User},
};

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    deleting_user: User,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User, deleting_user: User) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            deleting_user,
        })
    }
}

#[get("/users/delete/<id>", rank = 0)]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Template, Status> {
    let deleting_user = database.run(move |c| User::get_by_id(c, id)).await?;

    let user = administrator.0;
    let context = LayoutContext::new(language, user, deleting_user).await?;
    Ok(Template::render(
        "routes/administrator/users/delete",
        context,
    ))
}

#[post("/users/delete/<id>", rank = 0)]
pub async fn post(
    _administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Redirect, Status> {
    let deleting_user = database.run(move |c| User::get_by_id(c, id)).await?;

    database
        .run(move |c| deleting_user.update_deleted(c, true))
        .await?;

    Ok(Redirect::to("/users"))
}
