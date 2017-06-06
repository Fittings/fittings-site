use diesel::sqlite::SqliteConnection; //sqlite specific.
use dotenv::dotenv;
use std::env;
use r2d2::{PooledConnection, Pool, Config };
use r2d2_diesel::ConnectionManager;



pub mod schema;



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



pub fn get_db_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    match DB_CON_POOL.get() {
        Ok(conn) => conn,
        Err(e) => panic!("{}", e),
    }
}

