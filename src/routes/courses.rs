use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext, course::Course, error::Error, localization::Script,
    user::User,
};

#[derive(Serialize, Debug)]
pub struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    courses: Vec<Course>,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User, courses: Vec<Course>) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            courses,
        })
    }
}
