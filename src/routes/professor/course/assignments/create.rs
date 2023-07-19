use rocket::{form::Form, get, http::Status, post, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    assignment::{AssignmentType, GradeAssignment, PointAssignment},
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
    course: Course,
    show_success_message: bool,
    show_error_all_fields_required: bool,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User, course: Course) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            course,
            show_success_message: false,
            show_error_all_fields_required: false,
        })
    }

    pub fn success(mut self) -> Self {
        self.show_success_message = true;
        self
    }

    pub fn error_all_fields_required(mut self) -> Self {
        self.show_error_all_fields_required = true;
        self
    }
}

#[get("/course/<course>/assignments/create?<assignment_type>", rank = 1)]
pub async fn get(
    language: Script,
    database: Database,
    professor: Professor<'_>,
    course: String,
    assignment_type: Option<AssignmentType>,
) -> Result<Template, Status> {
    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    let user = professor.0;

    if !course.authorized_to_edit(user) {
        return Err(Status::Unauthorized);
    }

    match assignment_type {
        None => Ok(Template::render(
            "routes/professor/course/assignments/create",
            LayoutContext::new(language, user, course).await?,
        )),
        Some(AssignmentType::Grade) => Ok(Template::render(
            "routes/professor/course/assignments/create/grade",
            LayoutContext::new(language, user, course).await?,
        )),

        Some(AssignmentType::Point) => Ok(Template::render(
            "routes/professor/course/assignments/create/point",
            LayoutContext::new(language, user, course).await?,
        )),
    }
}

#[derive(FromForm, Debug)]
pub struct FormDataGrade {
    name: String,
}

// TODO: This is ranked because the compiler complains about a collision
// with the assignment_type=Point handler - there shouldn't be a collision,
// as they match different URLs
#[post(
    "/course/<course>/assignments/create?assignment_type=Grade",
    data = "<form>",
    rank = 1
)]
pub async fn post_grade(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    course: String,
    form: Form<FormDataGrade>,
) -> Result<Template, Status> {
    let user = professor.0;
    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    if !course.authorized_to_edit(user) {
        return Err(Status::Unauthorized);
    }

    if form.name.is_empty() {
        return Ok(Template::render(
            "routes/professor/course/assignments/create/grade",
            LayoutContext::new(language, user, course)
                .await?
                .error_all_fields_required(),
        ));
    }

    let url = crate::util::string_to_url(&form.name);

    database
        .run(move |c| GradeAssignment::create(c, course.id, &form.name, &url))
        .await?;

    Ok(Template::render(
        "routes/professor/course/assignments/create/grade",
        LayoutContext::new(language, user, course).await?.success(),
    ))
}

#[derive(FromForm, Debug)]
pub struct FormDataPoint {
    name: String,
    max_points: u32,
}

#[post(
    "/course/<course>/assignments/create?assignment_type=Point",
    data = "<form>",
    rank = 0
)]
pub async fn post_point(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    course: String,
    form: Form<FormDataPoint>,
) -> Result<Template, Status> {
    let user = professor.0;
    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    if !course.authorized_to_edit(user) {
        return Err(Status::Unauthorized);
    }

    if form.name.is_empty() || form.max_points == 0 {
        return Ok(Template::render(
            "routes/professor/course/assignments/create/point",
            LayoutContext::new(language, user, course)
                .await?
                .error_all_fields_required(),
        ));
    }

    let url = crate::util::string_to_url(&form.name);

    database
        .run(move |c| PointAssignment::create(c, course.id, &form.name, &url, form.max_points))
        .await?;

    Ok(Template::render(
        "routes/professor/course/assignments/create/point",
        LayoutContext::new(language, user, course).await?.success(),
    ))
}
