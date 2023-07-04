#![warn(clippy::pedantic)]
//#![deny(warnings)]

mod assignment;
mod base_layout_context;
mod course;
mod database;
mod error;
mod index;
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
use routes::login;
use routes::professor;
use routes::root;
use routes::student;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    let handlebars = Template::custom(|engines| {
        engines.handlebars.register_helper(
            localization::ScriptHelper::name(),
            localization::ScriptHelper::helper(),
        )
    });

    build()
        .mount("/", FileServer::from("static"))
        .mount(
            "/",
            routes![
                root::get,
                login::get,
                login::post,
                student::courses::get,
                professor::courses::get,
                professor::courses::create::get,
                professor::courses::create::post,
                professor::course::get,
                professor::course::delete::get,
                professor::course::delete::post,
                administrator::courses::get,
                administrator::users::get,
                administrator::users::delete::get,
                administrator::users::delete::post,
                administrator::users::create::get,
                administrator::users::create::post,
                administrator::users::edit::get,
                administrator::users::edit::post,
                administrator::generations::get,
                administrator::generations::post,
                administrator::generations::delete::get,
                administrator::generations::delete::post,
                administrator::programs::get,
                administrator::programs::post,
                administrator::programs::delete::get,
                administrator::programs::delete::post,
            ],
        )
        .attach(handlebars)
        .attach(Database::fairing())
        .manage(Mail::new().unwrap())
}
