use self::models::{Gallery, GalleryImage};
use images::models::{ImageLocation};
use diesel::prelude::*;
use database;
use database::schema::galleries::dsl::*;
use database::schema::{gallery_images, image_locations};



pub mod models;


/// Loads all Image Galleries in the database.
/// Doesn't contain any actual images.
pub fn get_all_image_galleries() -> Option<Vec<Gallery>>
{
    let conn = &*database::get_db_connection();

    let stored_galleries = match galleries.load::<Gallery>(conn) {
        Ok(vec) => vec,
        Err(_) => return None,
    };

    Some(stored_galleries)
}

/// Loads all the Image Locations for a Gallery
pub fn get_images_in_gallery(gallery_id : i32) -> Option<Vec<ImageLocation>> {
    let conn = &*database::get_db_connection();

    let images = match image_locations::table
        .inner_join(gallery_images::table)
        .filter(gallery_images::gallery_id.eq(gallery_id))
        .load::<(ImageLocation, GalleryImage)>(conn) {
        Ok(result) => result,
        Err(_) => return None,
    };

    let mut image_locations = Vec::new();
    for img in images {
        image_locations.push(img.0) //Only collect the ImageLocations
    }

    match image_locations.is_empty() {
        true => None,
        false => Some(image_locations),
    }
}
