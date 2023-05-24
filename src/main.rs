#![warn(clippy::pedantic)]
//#![deny(warnings)]

mod base_layout_context;
mod database;
mod error;
mod localization;
mod mail;
mod models;
mod routes;
mod schema;
mod user;

use database::Database;
use mail::Mail;
use rocket::fs::FileServer;
use rocket::{build, launch, routes};
use rocket_dyn_templates::Template;

use routes::administrator;
use routes::index;
use routes::login;
use routes::professor;
use routes::student;

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
                student::overview::get,
                professor::overview::get,
                administrator::overview::get,
                administrator::users::get,
                administrator::users::delete::get,
                administrator::users::delete::post,
                administrator::users::create::get,
                administrator::users::create::post,
                administrator::users::edit::get,
                administrator::users::edit::post
            ],
        )
        .attach(Template::fairing())
        .attach(Database::fairing())
        .attach(Mail::fairing())
}
