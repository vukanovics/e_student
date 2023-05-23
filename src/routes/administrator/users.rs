pub mod delete;
pub mod create;

use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    localization::Language,
    models::AccountType,
    user::{Administrator, User},
};

#[derive(Clone, Serialize, Debug)]
struct UserInfo {
    id: u32,
    username: Option<String>,
    email: String,
    account_type: AccountType,
}

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    users: Vec<UserInfo>,
}

impl LayoutContext {
    pub async fn new(
        language: Language,
        user: Option<&User>,
        users: Vec<UserInfo>,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            users,
        })
    }
}

#[get("/users", rank = 0)]
pub async fn get(
    language: Language,
    administrator: Administrator<'_>,
    database: Database,
) -> Result<Template, Status> {
    let user = administrator.0;

    let users = database.run(move |c| Database::get_all_users(c)).await?;

    let users = users
        .into_iter()
        .map(|user| UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            account_type: user.account_type,
        })
        .collect();

    let context = LayoutContext::new(language, Some(user), users).await?;

    Ok(Template::render("routes/administrator/users", context))
}
