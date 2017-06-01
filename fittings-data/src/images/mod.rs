use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use diesel;
use diesel::*;
use self::models::{ImageLocation, SubmitImageLocation};
use database::schema::image_locations::dsl::*;
use super::database;



pub mod models;

static IMAGE_PATH : &'static str = "./static/media";




pub fn load_image(file_name: String) -> Option<File> {
    let conn = database::get_db_connection();

    let image_location : Result<ImageLocation, diesel::result::Error> = image_locations.filter(name.eq(file_name))
        .first::<ImageLocation>(&*conn);

    match image_location {
        Ok(location) => match File::open(location.url) {
            Ok(file) => Some(file),
            _ => None,
        },
        _ => None,
    }
}


/// Uploads the image to the server with the given filename.
/// Returns the identifier of the image.
pub fn upload_image(image_name: String, image: Vec<u8>) -> io::Result<String> {
    let file_path : String = format!("{path}/{filename}", path = IMAGE_PATH, filename = image_name);
    let mut file = File::create(Path::new(file_path.as_str())).unwrap();
    file.write(image.as_slice()).unwrap();

    let image_location = SubmitImageLocation {
        name: image_name,
        url: file_path,
    };

    let conn = database::get_db_connection();
    diesel::insert(&image_location)
        .into(image_locations)
        .execute(&*conn)
        .expect("upload image failed to insert");

    Ok(image_location.url)
}