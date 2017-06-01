use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use diesel;
use diesel::*;
use self::models::SubmitImageLocation;
use database::schema::image_locations::dsl::*;
use super::database;



pub mod models;



//ZZZ TODO Improve error handling here.
/// Uploads the image to the server with the given filename.
/// Returns the identifier of the image.
pub fn upload_image(file_name: String, image: Vec<u8>) -> io::Result<String> {

    let file_path : String = format!("./static/media/{}", &file_name);
    let mut file = File::create(Path::new(file_path.as_str())).unwrap();
    file.write(image.as_slice()).unwrap();

    let image_location = SubmitImageLocation {
        url: file_name,
    };

    let conn = database::get_db_connection();
    diesel::insert(&image_location)
        .into(image_locations)
        .execute(&*conn)
        .expect("upload image failed to insert");

    Ok(image_location.url)
}