use rocket::{FromForm, FromFormField};
use serde::Serialize;

use crate::{
    database::{Database, SortDirection},
    error::Error,
    index::IndexNumber,
    user::{AccountType, UserWithIndex, Users, UsersRetrievalOptions},
};

#[derive(Serialize, Debug)]
pub struct LayoutContext {
    users: Vec<UserWithIndex>,
    form: Option<FormData>,
    number_of_pages: u32,
}

#[derive(Serialize, FromFormField, Debug, Clone)]
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

#[derive(Serialize, FromForm, Debug, Clone)]
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

    page: u32,
    max_per_page: u32,
}

const DEFAULT_USERS_PER_PAGE: u32 = 10;

impl LayoutContext {
    pub async fn new(database: Database, form: Option<FormData>) -> Result<LayoutContext, Error> {
        let mut options = UsersRetrievalOptions::new(0, DEFAULT_USERS_PER_PAGE);

        if let Some(form) = &form {
            options.filters.filter_email =
                Some(form.filter_email.clone()).filter(|s| !s.is_empty());

            options.filters.filter_account_type =
                Some(form.filter_account_type).filter(|_| form.filter_account_type_enabled);

            options.filters.filter_first_name =
                Some(form.filter_first_name.clone()).filter(|s| !s.is_empty());
            options.filters.filter_last_name =
                Some(form.filter_last_name.clone()).filter(|s| !s.is_empty());
            options.filters.filter_program =
                Some(form.filter_program.clone()).filter(|s| !s.is_empty());
            options.filters.filter_index_number = form.filter_index_number;
            options.filters.filter_generation = form.filter_generation;

            options.sort_by_first_name = (&form.sort_first_name).into();
            options.sort_by_last_name = (&form.sort_last_name).into();
            options.sort_by_email = (&form.sort_email).into();
            options.sort_by_account_type = (&form.sort_account_type).into();
            options.sort_by_index = (&form.sort_index).into();

            options.page = form.page;
            options.max_per_page = form.max_per_page;
        }

        let filters = options.filters.clone();
        let max_per_page = options.max_per_page;
        let users = database.run(move |c| Users::get_all(c, options)).await?;
        let number_of_pages = database
            .run(move |c| Users::get_number_of_pages(c, filters, max_per_page))
            .await?;
        Ok(LayoutContext {
            users: users.0,
            form,
            number_of_pages,
        })
    }
}
