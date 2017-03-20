#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate fittings_server;

use fittings_server::examples;

fn main() {
    let fittings_rocket = rocket::ignite();

    let mut routes = examples::get_routes("/");
    routes.extend(routes![index, get_fav_icon, get_main_css]);

    fittings_rocket.mount("/", routes).launch();
}




//ZZZ TODO Move static files out
use std::io;
use std::path::Path;
use rocket::response::NamedFile;


#[get("/favicon.ico")]
fn get_fav_icon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.png")).ok()
}

#[get("/main.css")]
fn get_main_css() -> io::Result<NamedFile> {
    NamedFile::open("static/main.css")
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
