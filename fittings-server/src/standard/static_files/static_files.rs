use rocket;
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    rocket.mount(base_address, routes![get_static_files, get_fav_icon, index])
}



#[get("/static/<file..>")]
fn get_static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


#[get("/favicon.ico")]
fn get_fav_icon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).ok()
}


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
