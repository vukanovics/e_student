use rocket::{form::Form, get, http::Status, post, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    assignment::{Assignment, GradeAssignment, PointAssignment},
    base_layout_context::BaseLayoutContext,
    components::users::{self, ControlTypeOptions, GradeProgressOptions, PointProgressOptions},
    course::Course,
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
    course: Course,
    assignment: Assignment,
}

impl LayoutContext {
    pub async fn new(
        language: Script,
        user: &User,
        users: users::LayoutContext,
        course: Course,
        assignment: Assignment,
    ) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            users,
            course,
            assignment,
        })
    }
}

#[get("/course/<course>/assignment/<assignment>/progress")]
pub async fn get(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    course: String,
    assignment: String,
) -> Result<Template, Status> {
    let user = professor.0;

    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    let assignment = database
        .run(move |c| Assignment::get(c, course.id, &assignment))
        .await?;

    let options = match &assignment {
        Assignment::Point(assignment) => ControlTypeOptions::PointProgress(PointProgressOptions {
            assignment: assignment.data.id,
        }),
        Assignment::Grade(assignment) => ControlTypeOptions::GradeProgress(GradeProgressOptions {
            assignment: assignment.data.id,
        }),
    };

    let users_context = users::LayoutContext::new(database, None, options).await?;
    let context = LayoutContext::new(language, user, users_context, course, assignment).await?;

    Ok(Template::render(
        "routes/professor/course/assignment/progress",
        context,
    ))
}

#[derive(Serialize, FromForm, Debug)]
pub struct FormData {
    users_form: users::FormData,
}

#[post("/course/<course>/assignment/<assignment>/progress", data = "<form>")]
pub async fn post(
    language: Script,
    professor: Professor<'_>,
    database: Database,
    form: Form<FormData>,
    course: String,
    assignment: String,
) -> Result<Template, Status> {
    let user = professor.0;

    let course = database
        .run(move |c| Course::get_by_url(c, &course))
        .await?;

    let assignment = database
        .run(move |c| Assignment::get(c, course.id, &assignment))
        .await?;

    let options = match &assignment {
        Assignment::Point(assignment) => {
            for point_progress in form.users_form.point_progresses() {
                if !point_progress.value_changed() {
                    continue;
                }

                let user = point_progress.user();
                let points = point_progress.new_value().unwrap_or_default();
                let assignment_id = assignment.data.id;

                database
                    .run(move |c| PointAssignment::grade(c, assignment_id, user, points))
                    .await?;
            }

            ControlTypeOptions::PointProgress(PointProgressOptions {
                assignment: assignment.data.id,
            })
        }
        Assignment::Grade(assignment) => {
            for grade_progress in form.users_form.grade_progresses() {
                if !grade_progress.value_changed() {
                    continue;
                }

                let user = grade_progress.user();
                let grade = grade_progress.new_value().unwrap_or_default();
                let assignment_id = assignment.data.id;

                database
                    .run(move |c| GradeAssignment::grade(c, assignment_id, user, grade))
                    .await?;
            }

            ControlTypeOptions::GradeProgress(GradeProgressOptions {
                assignment: assignment.data.id,
            })
        }
    };

    let users_context =
        users::LayoutContext::new(database, Some(form.into_inner().users_form), options).await?;
    let context = LayoutContext::new(language, user, users_context, course, assignment).await?;

    Ok(Template::render(
        "routes/professor/course/assignment/progress",
        context,
    ))
}
