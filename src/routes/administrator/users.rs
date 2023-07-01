pub mod create;
pub mod delete;
pub mod edit;

use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use serde::Serialize;
use diesel::prelude::*;

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
    users: Vec<User>,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User, users: Vec<User>) -> Result<Self, Error> {
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

    let users = database
        .run(move |c| {
            use crate::schema::users;
            users::table.load::<User>(c).map_err(Error::from)
        })
        .await?;

    let context = LayoutContext::new(language, user, users).await?;

    Ok(Template::render("routes/administrator/users", context))
}
