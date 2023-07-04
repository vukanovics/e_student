pub mod delete;

use rocket::{form::Form, get, http::Status, post, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    index::{Program, Programs},
    localization::Script,
    user::{Administrator, User},
};

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    programs: Vec<Program>,
    show_success_message: bool,
    show_error_duplicate_name: bool,
    show_error_short_name_too_long: bool,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User, database: Database) -> Result<Self, Error> {
        let programs = database.run(|c| Programs::get(c)).await?.0;
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            programs,
            show_success_message: false,
            show_error_duplicate_name: false,
            show_error_short_name_too_long: false,
        })
    }

    pub fn show_success_message(mut self) -> Self {
        self.show_success_message = true;
        self
    }

    pub fn show_error_duplicate_name(mut self) -> Self {
        self.show_error_duplicate_name = true;
        self
    }

    pub fn show_error_short_name_too_long(mut self) -> Self {
        self.show_error_short_name_too_long = true;
        self
    }
}

#[get("/programs")]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
) -> Result<Template, Status> {
    let user = administrator.0;

    Ok(Template::render(
        "routes/administrator/programs",
        LayoutContext::new(language, user, database).await?,
    ))
}

#[derive(FromForm, Debug)]
pub struct FormData {
    short_name: String,
    full_name: String,
}

#[post("/programs", data = "<form>")]
pub async fn post(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    form: Form<FormData>,
) -> Result<Template, Status> {
    let user = administrator.0;

    // Short name can be upto 2 characters long
    // TODO: This should be handled as an Error variant when trying to
    // insert to the database. Currently this is difficult, as 'too long'
    // SQL error gets translated into Unknown variant by Diesel.
    if form.short_name.len() > 2 {
        return Ok(Template::render(
            "routes/administrator/programs",
            LayoutContext::new(language, user, database)
                .await?
                .show_error_short_name_too_long(),
        ));
    }

    match database
        .run(move |c| Program::create(c, form.short_name.clone(), form.full_name.clone()))
        .await
    {
        Ok(_) => Ok(Template::render(
            "routes/administrator/programs",
            LayoutContext::new(language, user, database)
                .await?
                .show_success_message(),
        )),
        Err(Error::DatabaseDuplicateEntry) => Ok(Template::render(
            "routes/administrator/programs",
            LayoutContext::new(language, user, database)
                .await?
                .show_error_duplicate_name(),
        )),
        Err(e) => Err(e.into()),
    }
}
