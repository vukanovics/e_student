use diesel::Connection;
use rocket::{form::Form, get, http::Status, post, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    components::users::{self, ControlTypeOptions, EnrolOptions},
    course::{Course, Enrolment},
    database::Database,
    error::Error,
    localization::Script,
    user::{Professor, User},
};

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    users: users::LayoutContext,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        users: users::LayoutContext,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            users,
        })
    }
}

#[get("/course/<course>/enrol")]
pub async fn get(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    course: String,
) -> Result<Template, Status> {
    let user = professor.0;

    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    if !course.authorized_to_edit(user) {
        return Err(Status::Unauthorized);
    }

    let options = ControlTypeOptions::Enrol(EnrolOptions { course: course.id });

    let users_context = users::LayoutContext::new(database, None, options).await?;
    let context = LayoutContext::new(language, user, users_context).await?;

    Ok(Template::render("routes/professor/course/enrol", context))
}

#[derive(Serialize, FromForm, Debug)]
pub struct FormData {
    users_form: users::FormData,
}

#[post("/course/<course>/enrol", data = "<form>")]
pub async fn post(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    form: Form<FormData>,
    course: String,
) -> Result<Template, Status> {
    let user = professor.0;

    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    if !course.authorized_to_edit(user) {
        return Err(Status::Unauthorized);
    }

    for dropdown in form.users_form.enrol_dropdowns() {
        if dropdown.value_changed() {
            let student = dropdown.user();
            match *dropdown.new_value() {
                true => {
                    database
                        .run(move |c| Enrolment::create(c, course.id, student))
                        .await?
                }
                false => {
                    database
                        .run(move |c| {
                            c.transaction(move |c| Enrolment::get(c, course.id, student)?.delete(c))
                        })
                        .await?
                }
            }
        }
    }

    let options = ControlTypeOptions::Enrol(EnrolOptions { course: course.id });

    let users_context =
        users::LayoutContext::new(database, Some(form.into_inner().users_form), options).await?;
    let context = LayoutContext::new(language, user, users_context).await?;

    Ok(Template::render("routes/professor/course/enrol", context))
}
