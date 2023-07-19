use rocket::{
    get,
    http::{Cookie, CookieJar},
    response::Redirect,
};

use crate::user::SESSION_KEY_COOKIE_NAME;

#[get("/logout")]
pub async fn get(jar: &CookieJar<'_>) -> Redirect {
    jar.remove(Cookie::named(SESSION_KEY_COOKIE_NAME));
    Redirect::to("/login")
}
