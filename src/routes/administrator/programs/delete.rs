use rocket::{get, http::Status, post, response::Redirect};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    index::Program,
    localization::Script,
    user::{Administrator, User},
};

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    deleting_program: Program,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        deleting_program: Program,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            deleting_program,
        })
    }
}

#[get("/programs/delete/<id>")]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Template, Status> {
    let deleting_program = database.run(move |c| Program::get_by_id(c, id)).await?;

    let user = administrator.0;
    let context = LayoutContext::new(language, user, deleting_program).await?;
    Ok(Template::render(
        "routes/administrator/programs/delete",
        context,
    ))
}

#[post("/programs/delete/<id>", rank = 0)]
pub async fn post(
    _administrator: Administrator<'_>,
    database: Database,
    id: u32,
) -> Result<Redirect, Status> {
    let deleting_program = database.run(move |c| Program::get_by_id(c, id)).await?;

    database.run(move |c| deleting_program.delete(c)).await?;

    Ok(Redirect::to("/programs"))
}
