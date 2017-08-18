use self::models::{NewUser, StoredUser};
use database;
use diesel::result::Error;
use diesel;
use diesel::prelude::*;
use database::schema::users;


pub mod models;



/// Creates a new user in the database.
/// Usernames are unique and will return a Result(Err) if there is a duplicate.
/// Hash and Salt should be generated in the fittings_server::security module.
pub fn create_user(username: String, hash: String, salt: String) -> Result<i32, Error> {
    let conn = &*database::get_db_connection();

    let new_user = NewUser {
        username: username,
        hash: hash,
        salt: salt,
    };

    match diesel::insert(&new_user)
        .into(users::table)
        .execute(conn) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match users::table
        .order(users::id.desc())
        .first::<StoredUser>(conn) {
        Ok(user) => Ok(user.id),
        Err(e) => Err(e),
    }
}


pub fn get_stored_user_data(stored_username: &String) -> Option<StoredUser> {
    let conn = &*database::get_db_connection();

    match users::table.filter(users::username.eq(stored_username)).
        first::<StoredUser>(&*conn) {
        Ok(stored_user) => Some(stored_user),
        Err(_) => None,
    }
}