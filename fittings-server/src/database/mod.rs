//ZZZ TODO Move imports & code into their own module.
pub mod schema;
pub mod models;



use diesel::prelude::*;
use diesel::sqlite::SqliteConnection; //sqlite specific.
use dotenv::dotenv;
use std::env;

extern crate diesel;


use r2d2::{ Pool, Config };
use r2d2_diesel::ConnectionManager;

lazy_static! {
    pub static ref DB_CON_POOL: Pool<ConnectionManager<SqliteConnection>> = create_db_pool();
}

fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file.");
    let config = Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::new(config, manager).expect("Failed to create pool.")
}


use r2d2::PooledConnection;
pub fn get_db_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    match DB_CON_POOL.get() {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e),
    }
}



use self::models::SubmitImageLocation;
pub fn create_image(conn: &SqliteConnection, image_loc: String) {
    use self::schema::image_locations;

    let new_image = SubmitImageLocation {
        url: image_loc,
    };

    let result = diesel::insert(&new_image).into(image_locations::table).execute(conn);
    println!("{}", result.unwrap());
}
