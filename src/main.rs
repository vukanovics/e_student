#![warn(clippy::pedantic)]
#![deny(warnings)]

mod application;
mod base_layout_context;
mod database;
mod error;
mod localization;
mod models;
mod routes;
mod schema;

use database::Database;
use rocket::fs::FileServer;
use rocket::{build, launch, routes};
use rocket_dyn_templates::Template;

use routes::course;
use routes::index;
use routes::login;
use routes::overview;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    build()
        .mount("/", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index::get,
                login::get,
                login::post,
                course::get,
                overview::get
            ],
        )
        .attach(Template::fairing())
        .attach(Database::fairing())
}
