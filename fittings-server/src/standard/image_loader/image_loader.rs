use rocket;
use rocket::response::NamedFile;
use rocket::State;
use std::sync::{Arc, Mutex};

pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    rocket.mount(base_address, routes![get_dynamic_images])
}

use database;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
#[get("/image/<image_id>")]
fn get_dynamic_images(image_id: String, conn: State<Arc<Mutex<SqliteConnection>>>) -> Option<NamedFile> {
    print!("{}", image_id);

    use database::schema::images::dsl::*;
    images.load::<database::models::Image>(&*conn.lock().unwrap()).expect("Error loading images");

    None
}
