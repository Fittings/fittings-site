//ZZZ TODO Move imports & code into their own module.



pub mod schema;
pub mod models;


use diesel::prelude::*;
use diesel::sqlite::SqliteConnection; //sqlite specific.
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file.");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
