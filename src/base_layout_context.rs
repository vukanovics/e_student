use crate::{database::Database, error::Error};
use rocket::http::CookieJar;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BaseLayoutContext {}

impl BaseLayoutContext {
    pub fn new(_database: &Database, _jar: &CookieJar) -> Result<Self, Error> {
        Ok(Self {})
    }
}
