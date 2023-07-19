use rocket::{
    get,
    http::{Cookie, CookieJar},
    response::Redirect,
};

#[get("/locale/<code>?<redirect>")]
pub async fn get(code: String, jar: &CookieJar<'_>, redirect: Option<String>) -> Redirect {
    jar.add(Cookie::new("language", code));
    Redirect::to(redirect.unwrap_or("/".to_string()))
}
