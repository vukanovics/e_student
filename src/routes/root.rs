use rocket::{get, response::Redirect};

#[get("/")]
pub async fn get() -> Redirect {
    Redirect::permanent("/login")
}
