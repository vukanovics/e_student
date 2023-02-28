#![warn(clippy::pedantic)]
#![deny(warnings)]

mod routes;

use rocket::{build, launch, routes};
use rocket_dyn_templates::Template;

use routes::index;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    build()
        .mount("/", routes![index::get])
        .attach(Template::fairing())
}
