use rocket::{form::Form, get, http::Status, post, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    course::Course,
    database::Database,
    error::Error,
    localization::Script,
    user::{Professor, User},
};

#[derive(Clone, Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    show_success_message: bool,
    show_course_name_is_required: bool,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            show_success_message: false,
            show_course_name_is_required: false,
        })
    }

    pub fn success(mut self) -> Self {
        self.show_success_message = true;
        self
    }

    pub fn course_name_is_required(mut self) -> Self {
        self.show_course_name_is_required = true;
        self
    }
}

#[get("/courses/create", rank = 0)]
pub async fn get(language: Script, professor: Professor<'_>) -> Result<Template, Status> {
    let user = professor.0;
    Ok(Template::render(
        "routes/professor/courses/create",
        LayoutContext::new(language, user).await?,
    ))
}

#[derive(FromForm, Debug)]
pub struct FormData {
    year: u32,
    name: String,
}

#[post("/courses/create", data = "<form>", rank = 0)]
pub async fn post(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    form: Form<FormData>,
) -> Result<Template, Status> {
    let user = professor.0;

    if form.name.is_empty() {
        return Ok(Template::render(
            "routes/professor/courses/create",
            LayoutContext::new(language, user)
                .await?
                .course_name_is_required(),
        ));
    }

    let url: String = crate::util::string_to_url(&form.name);

    println!("Url is {:?}", url);

    let user_id = user.id();

    database
        .run(move |c| Course::create(c, form.year, &form.name, &url, user_id))
        .await?;

    Ok(Template::render(
        "routes/professor/courses/create",
        LayoutContext::new(language, user).await?.success(),
    ))
}
