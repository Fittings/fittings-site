#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate fittings_server;

use fittings_server::examples;

//ZZZ TODO Move static files out
use std::path::Path;
use rocket::response::NamedFile;

#[get("/favicon.ico")]
fn get_fav_icon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.png")).ok()
}

fn main() {
    let fittings_rocket = rocket::ignite();

    let mut routes = examples::get_routes("/");
    routes.extend(routes![get_fav_icon]);

    fittings_rocket.mount("/", routes).launch();
}
