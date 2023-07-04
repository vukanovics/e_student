pub mod delete;

use rocket::{form::Form, get, http::Status, post, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    index::{Generation, Generations},
    localization::Script,
    user::{Administrator, User},
};

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    generations: Vec<Generation>,
    show_success_message: bool,
    show_error_duplicate_year: bool,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User, database: Database) -> Result<Self, Error> {
        let generations = database.run(|c| Generations::get(c)).await?.0;
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            generations,
            show_success_message: false,
            show_error_duplicate_year: false,
        })
    }

    pub fn show_success_message(mut self) -> Self {
        self.show_success_message = true;
        self
    }

    pub fn show_error_duplicate_year(mut self) -> Self {
        self.show_error_duplicate_year = true;
        self
    }
}

#[get("/generations")]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
) -> Result<Template, Status> {
    let user = administrator.0;

    Ok(Template::render(
        "routes/administrator/generations",
        LayoutContext::new(language, user, database).await?,
    ))
}

#[derive(FromForm, Debug)]
pub struct FormData {
    year: u32,
}

#[post("/generations", data = "<form>")]
pub async fn post(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    form: Form<FormData>,
) -> Result<Template, Status> {
    let user = administrator.0;

    match database
        .run(move |c| Generation::create(c, form.year))
        .await
    {
        Ok(_) => Ok(Template::render(
            "routes/administrator/generations",
            LayoutContext::new(language, user, database)
                .await?
                .show_success_message(),
        )),
        Err(Error::DatabaseDuplicateEntry) => Ok(Template::render(
            "routes/administrator/generations",
            LayoutContext::new(language, user, database)
                .await?
                .show_error_duplicate_year(),
        )),
        Err(e) => Err(e.into()),
    }
}
