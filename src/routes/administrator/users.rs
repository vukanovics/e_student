pub mod create;
pub mod delete;
pub mod edit;

use log::debug;
use rocket::{form::Form, get, http::Status, post, FromForm, FromFormField};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::{Database, SortDirection},
    error::Error,
    index::IndexNumber,
    localization::Script,
    user::{AccountType, Administrator, User, UserWithIndex, Users, UsersRetrievalOptions},
};

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    form: Option<FormData>,
    users: Vec<UserWithIndex>,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        users: Vec<UserWithIndex>,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            form: None,
            users,
        })
    }

    pub fn with_form(mut self, form: Option<FormData>) -> Self {
        self.form = form;
        self
    }
}

#[get("/users", rank = 0)]
pub async fn get(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
) -> Result<Template, Status> {
    let user = administrator.0;

    let options = UsersRetrievalOptions::new();

    let users = database.run(move |c| Users::get_all(c, options)).await?;

    let context = LayoutContext::new(language, user, users.0).await?;

    Ok(Template::render("routes/administrator/users", context))
}

#[derive(Serialize, FromFormField, Debug)]
pub enum FormSortDirection {
    None,
    Ascending,
    Descending,
}

impl Into<Option<SortDirection>> for &FormSortDirection {
    fn into(self) -> Option<SortDirection> {
        match self {
            FormSortDirection::None => None,
            FormSortDirection::Ascending => Some(SortDirection::Ascending),
            FormSortDirection::Descending => Some(SortDirection::Descending),
        }
    }
}

#[derive(Serialize, FromForm, Debug)]
pub struct FormData {
    filter_email: String,
    filter_account_type_enabled: bool,
    filter_account_type: AccountType,
    filter_first_name: String,
    filter_last_name: String,
    filter_index_number: Option<IndexNumber>,
    filter_program: String,
    filter_generation: Option<u32>,

    sort_first_name: FormSortDirection,
    sort_last_name: FormSortDirection,
    sort_email: FormSortDirection,
    sort_account_type: FormSortDirection,
    sort_index: FormSortDirection,
}

#[post("/users", data = "<form>", rank = 0)]
pub async fn post(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    form: Form<FormData>,
) -> Result<Template, Status> {
    let user = administrator.0;

    let mut options = UsersRetrievalOptions::new();

    debug!("Form={:?}", form);
    options.filter_email = Some(form.filter_email.clone()).filter(|s| !s.is_empty());

    options.filter_account_type =
        Some(form.filter_account_type).filter(|_| form.filter_account_type_enabled);

    options.filter_first_name = Some(form.filter_first_name.clone()).filter(|s| !s.is_empty());
    options.filter_last_name = Some(form.filter_last_name.clone()).filter(|s| !s.is_empty());
    options.filter_program = Some(form.filter_program.clone()).filter(|s| !s.is_empty());
    options.filter_index_number = form.filter_index_number;
    options.filter_generation = form.filter_generation;
    options.sort_by_first_name = (&form.sort_first_name).into();
    options.sort_by_last_name = (&form.sort_last_name).into();
    options.sort_by_email = (&form.sort_email).into();
    options.sort_by_account_type = (&form.sort_account_type).into();
    options.sort_by_index = (&form.sort_index).into();

    debug!("Options={:?}", options);

    let users = database.run(move |c| Users::get_all(c, options)).await?;

    let context = LayoutContext::new(language, user, users.0)
        .await?
        .with_form(Some(form.into_inner()));

    Ok(Template::render("routes/administrator/users", context))
}
