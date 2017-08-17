use self::models::{NewUser, StoredUser};
use database;
use diesel::result::Error;
use diesel;
use diesel::prelude::*;
use database::schema::galleries::dsl::*;
use database::schema::users;



pub mod models;



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