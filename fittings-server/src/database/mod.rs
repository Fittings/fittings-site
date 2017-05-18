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
    static ref COUNT: u32 = 21;
}

fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file.");
    let config = Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::new(config, manager).expect("Failed to create pool.")
}



use self::models::NewImage;
pub fn create_image(conn: &SqliteConnection, image: Vec<u8>) {
    use self::schema::images;

    let new_image = NewImage {
        image: image,
    };

    //ZZZ TODO What is the result object?
    let result = diesel::insert(&new_image).into(images::table).execute(conn);
    println!("{}", result.unwrap()); //result is: std::result::Result<usize, diesel::result::Error>
}
