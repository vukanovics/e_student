use rocket::{get, http::Status, post, response::Redirect};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    localization::Language,
    user::{Administrator, User},
};

use super::UserInfo;

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    deleting_user: UserInfo,
}

impl LayoutContext {
    pub async fn new(
        language: Language,
        user: &User,
        deleting_user: UserInfo,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            deleting_user,
        })
    }
}

#[get("/users/delete/<id>", rank = 0)]
pub async fn get(
    language: Language,
    administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Template, Status> {
    let deleting_user = database
        .run(move |c| Database::get_user_by_id(c, id))
        .await?;

    let deleting_user = UserInfo {
        id: deleting_user.id,
        username: deleting_user.username,
        email: deleting_user.email,
        account_type: deleting_user.account_type,
    };

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
    database
        .run(move |c| Database::delete_user_by_id(c, id))
        .await?;

    Ok(Redirect::to("/users"))
}
