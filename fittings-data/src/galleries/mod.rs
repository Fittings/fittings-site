use self::models::Gallery;
use diesel;
use diesel::prelude::*;
use database;
use database::schema::galleries::dsl::*;
use diesel::expression::dsl::*;

pub mod models;



pub fn get_all_image_galleries() -> Option<Vec<Gallery>>
{
    let conn = database::get_db_connection();

    let stored_galleries : Vec<Gallery> = match galleries.load::<Gallery>(&*conn) {
        Ok(vec) => vec,
        Err(_) => return None,
    };

    Some(stored_galleries)
}
