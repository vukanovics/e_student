use rocket::{FromForm, FromFormField};
use serde::Serialize;

use crate::{
    assignment::GradeAssignmentGrade,
    database::{Database, SortDirection},
    error::Error,
    index::IndexNumber,
    user::{
        AccountType, UserId, UserWithIndex, UserWithIndexAndEnrolment,
        UserWithIndexAndGradeProgress, UserWithIndexAndPointProgress, UsersRetrievalOptions,
        UsersWithIndex, UsersWithIndexAndEnrolment, UsersWithIndexAndGradeProgress,
        UsersWithIndexAndPointProgress,
    },
};

#[derive(Serialize, Debug)]
pub struct EditData {
    users: Vec<UserWithIndex>,
}

#[derive(Serialize, Debug)]
pub struct EnrolData {
    users: Vec<UserWithIndexAndEnrolment>,
}

#[derive(Serialize, Debug)]
pub struct PointProgressData {
    users: Vec<UserWithIndexAndPointProgress>,
}

#[derive(Serialize, Debug)]
pub struct GradeProgressData {
    users: Vec<UserWithIndexAndGradeProgress>,
}

#[derive(Serialize, Debug)]
pub enum ControlType {
    Edit(EditData),
    Enrol(EnrolData),
    PointProgress(PointProgressData),
    GradeProgress(GradeProgressData),
}

#[derive(Serialize, Debug)]
pub struct LayoutContext {
    form: Option<FormData>,
    number_of_pages: u32,
    control_type: ControlType,
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
pub struct UserValueDifference<T> {
    user: UserId,
    old_value: T,
    new_value: T,
}

impl<T> UserValueDifference<T>
where
    T: PartialEq,
{
    pub fn user(&self) -> UserId {
        self.user
    }

    pub fn value_changed(&self) -> bool {
        self.old_value != self.new_value
    }

    pub fn new_value(&self) -> &T {
        &self.new_value
    }
}

pub type PointProgress = UserValueDifference<Option<u32>>;
pub type GradeProgress = UserValueDifference<Option<GradeAssignmentGrade>>;
pub type EnrolDropdown = UserValueDifference<bool>;

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

    enrol_dropdowns: Vec<EnrolDropdown>,
    point_progresses: Vec<PointProgress>,
    grade_progresses: Vec<GradeProgress>,
}

impl FormData {
    pub fn enrol_dropdowns(&self) -> &Vec<EnrolDropdown> {
        self.enrol_dropdowns.as_ref()
    }
    pub fn point_progresses(&self) -> &Vec<PointProgress> {
        self.point_progresses.as_ref()
    }
    pub fn grade_progresses(&self) -> &Vec<GradeProgress> {
        self.grade_progresses.as_ref()
    }
}

pub struct EnrolOptions {
    pub course: u32,
}

pub struct PointProgressOptions {
    pub assignment: u32,
}

pub struct GradeProgressOptions {
    pub assignment: u32,
}

pub enum ControlTypeOptions {
    Edit,
    Enrol(EnrolOptions),
    PointProgress(PointProgressOptions),
    GradeProgress(GradeProgressOptions),
}

const DEFAULT_USERS_PER_PAGE: u32 = 10;

impl LayoutContext {
    pub async fn new(
        database: Database,
        form: Option<FormData>,
        control_type: ControlTypeOptions,
    ) -> Result<LayoutContext, Error> {
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

            options.sorts.sort_by_first_name = (&form.sort_first_name).into();
            options.sorts.sort_by_last_name = (&form.sort_last_name).into();
            options.sorts.sort_by_email = (&form.sort_email).into();
            options.sorts.sort_by_account_type = (&form.sort_account_type).into();
            options.sorts.sort_by_index = (&form.sort_index).into();

            options.page = form.page;
            options.max_per_page = form.max_per_page;
        }

        let filters = options.filters.clone();
        let max_per_page = options.max_per_page;
        let number_of_pages = database
            .run(move |c| UsersWithIndex::get_number_of_pages(c, filters, max_per_page))
            .await?;

        let control_type = match control_type {
            ControlTypeOptions::Edit => {
                let users = database
                    .run(move |c| UsersWithIndex::get(c, options))
                    .await?;
                ControlType::Edit(EditData { users: users.0 })
            }
            ControlTypeOptions::Enrol(settings) => {
                let users = database
                    .run(move |c| UsersWithIndexAndEnrolment::get(c, options, settings.course))
                    .await?;
                ControlType::Enrol(EnrolData { users: users.0 })
            }
            ControlTypeOptions::PointProgress(settings) => {
                let users = database
                    .run(move |c| {
                        UsersWithIndexAndPointProgress::get(c, options, settings.assignment)
                    })
                    .await?;
                ControlType::PointProgress(PointProgressData { users: users.0 })
            }
            ControlTypeOptions::GradeProgress(settings) => {
                let users = database
                    .run(move |c| {
                        UsersWithIndexAndGradeProgress::get(c, options, settings.assignment)
                    })
                    .await?;
                ControlType::GradeProgress(GradeProgressData { users: users.0 })
            }
        };

        Ok(LayoutContext {
            form,
            number_of_pages,
            control_type,
        })
    }
}
