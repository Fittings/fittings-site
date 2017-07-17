use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use diesel::result::Error;
use diesel;
use diesel::*;
use self::models::{ImageLocation, SubmitImageLocation};
use database::schema::image_locations::dsl::*;
use database::schema::image_locations;
use super::database;


pub mod models;

static IMAGE_PATH: &'static str = "/static/media";


pub fn load_image(file_name: String) -> Option<File> {
    let conn = database::get_db_connection();

    let image_location: ImageLocation = match image_locations.filter(name.eq(file_name)).
        first::<ImageLocation>(&*conn) {
        Ok(image_location) => image_location,
        Err(_) => return None,
    };

    match File::open(image_location.url) {
        Ok(file) => Some(file),
        Err(_) => None,
    }
}


/// Uploads the image to the server with the given filename.
/// Returns the identifier of the image.
pub fn store_image(image_name: String, image: Vec<u8>) -> io::Result<String> {
    let file_path: String = format!("{path}/{filename}", path = IMAGE_PATH, filename = image_name);
    let mut file = File::create(Path::new( format!(".{path}", path=file_path).as_str() )).unwrap();
    file.write(image.as_slice()).unwrap();

    Ok(file_path)
}


pub fn insert_image_location(image_name: String, image_path: String) -> Result<i32, Error> {
    let conn = &*database::get_db_connection();

    println!("inserting image path {}", image_path.clone());/

    let image_location = SubmitImageLocation { name: image_name, url: image_path };

    match diesel::insert(&image_location)
        .into(image_locations)
        .execute(conn) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    println!("Getting image id!");

    match image_locations::table
        .order(image_locations::id.desc())
        .first::<ImageLocation>(conn) {
        Ok(img_loc) => Ok(img_loc.id),
        Err(e) => Err(e),
    }
}