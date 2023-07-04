use rocket::{get, http::Status, post, response::Redirect};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    index::Generation,
    localization::Script,
    user::{Administrator, User},
};

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    deleting_generation: Generation,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        deleting_generation: Generation,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            deleting_generation,
        })
    }
}

#[get("/generations/delete/<id>")]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Template, Status> {
    let deleting_generation = database.run(move |c| Generation::get_by_id(c, id)).await?;

    let user = administrator.0;
    let context = LayoutContext::new(language, user, deleting_generation).await?;
    Ok(Template::render(
        "routes/administrator/generations/delete",
        context,
    ))
}

#[post("/generations/delete/<id>", rank = 0)]
pub async fn post(
    _administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Redirect, Status> {
    let deleting_generation = database.run(move |c| Generation::get_by_id(c, id)).await?;

    database.run(move |c| deleting_generation.delete(c)).await?;

    Ok(Redirect::to("/generations"))
}
