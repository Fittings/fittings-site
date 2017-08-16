use super::database;

pub mod models;



pub fn create_user(username: String, hash: String, salt: String) {
    let conn = database::get_db_connection();

    println!("creating user: {}", &username);
}