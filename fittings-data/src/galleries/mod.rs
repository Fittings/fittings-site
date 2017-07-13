use self::models::{NewGallery, Gallery, GalleryImage};
use images::models::{ImageLocation};
use diesel::prelude::*;
use diesel;
use diesel::result::Error;
use database;
use database::schema::galleries::dsl::*;
use database::schema::{galleries, gallery_images, image_locations};



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

pub fn get_first_image_in_gallery(gallery_id : i32) -> Option<ImageLocation> {
    let conn = &*database::get_db_connection();

    match image_locations::table
        .inner_join(gallery_images::table)
        .filter(gallery_images::gallery_id.eq(gallery_id))
        .first::<(ImageLocation, GalleryImage)>(conn) {
        Ok(result) => Some(result.0),
        Err(_) => None,
    }
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

    //ZZZ TODO Learn how to do this functional style.
    let mut image_locations = Vec::new();
    for img in images {
        image_locations.push(img.0) //Only collect the ImageLocations
    }

    match image_locations.is_empty() {
        true => None,
        false => Some(image_locations),
    }
}

pub fn create_gallery(gallery_name: String, gallery_desc: String) -> Result<i32, Error> {
    let conn = &*database::get_db_connection();

    let new_gallery = NewGallery {
        name: gallery_name,
        description: gallery_desc,
    };

    match diesel::insert(&new_gallery)
        .into(galleries::table)
        .execute(conn) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    //Need to do an extra call to get the inserted gallery id.
    match galleries::table
        .order(galleries::id.desc())
        .first::<Gallery>(conn) {
        Ok(gallery) => Ok(gallery.id),
        Err(e) => Err(e),
    }
}

pub fn insert_gallery_image(gallery_id: i32, image_id: i32) -> Result<(), Error> {
    let conn = &*database::get_db_connection();

    let gallery_image = GalleryImage {
        gallery_id : gallery_id,
        image_id : image_id,
    };

    match diesel::insert(&gallery_image)
        .into(gallery_images::table)
        .execute(conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
    }
}

