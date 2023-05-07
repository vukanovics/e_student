use rocket::{
    get,
    http::{CookieJar, Status},
};
use rocket_dyn_templates::Template;

use crate::{
    application::get_user_from_jar, database::Database, localization::Language, models::AccountType,
};

use super::administrator;
use super::professor;
use super::student;

#[get("/<language>/overview")]
pub async fn get(
    database: Database,
    jar: &CookieJar<'_>,
    language: &str,
) -> Result<Template, Status> {
    let user = get_user_from_jar(&database, jar).await?;
    let user = match user {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let language = Language::from_code(language)?;

    match user.account_type {
        AccountType::Student => student::overview::get(language, user, database, jar).await,
        AccountType::Professor => professor::overview::get(language, user, database, jar).await,
        AccountType::Administrator => {
            administrator::overview::get(language, user, database, jar).await
        }
    }
}
