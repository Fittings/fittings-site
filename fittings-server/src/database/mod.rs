//ZZZ TODO Move imports & code into their own module.
pub mod schema;
pub mod models;


use diesel::prelude::*;
use diesel::sqlite::SqliteConnection; //sqlite specific.
use dotenv::dotenv;
use std::env;

extern crate diesel;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file.");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}



use self::models::{Image, NewImage};
pub fn create_image(conn: &SqliteConnection, image: Vec<u8>) -> Vec<u8> {
    use self::schema::images;

    let new_image = NewImage {
        image: image,
    };

    diesel::insert(&new_image).into(images::table)
                              .get_results(conn)
                              .expect("Error saving image")
}
