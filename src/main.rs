#![warn(clippy::pedantic)]
#![deny(warnings)]

mod base_layout_context;
mod database;
mod error;
mod models;
mod routes;
mod schema;

use database::Database;
use rocket::{build, launch, routes};
use rocket_dyn_templates::Template;

use routes::index;
use routes::login;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    build()
        .mount("/", routes![index::get, login::get, login::post])
        .attach(Template::fairing())
        .attach(Database::fairing())
}
