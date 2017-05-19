# Setting up a web server in Rust.
My notes on how to setup my web server for [Fittings.net.nz](https://fittings.net.nz).

These notes are specifically so I can easily replicate the setup process.

**VPS:**
* Digital Ocean

**OS:**
* Windows + [Bash/WSL](https://msdn.microsoft.com/en-us/commandline/wsl/about)
* Ubuntu

**Languages:**
* [Rust](https://www.rust-lang.org/)
  * [Rocket](https://rocket.rs/)
  * [Diesel](https://diesel.rs/)
* Javascript
  * [React](https://facebook.github.io/react/)
* SQLite


# Rust Server 

## Rocket Installation

## Module Seperation

## Basic static file serving.



# Database

## sqlite3 Setup

## diesel setup

## Support for multiple connections.
To support multiple threads accessing the database we setup a connection pool.

Add to your rust server the following crates:
[lazy_static](https://crates.io/crates/lazy_static),
[r2d2](https://crates.io/crates/r2d2),
[r2d2-diesel](https://crates.io/crates/r2d2-diesel)


In the database module add the following code. This will allow
```rust
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
```



# Client-Side

## ReactJS Setup.

# Reverse Proxy and HTTPS setup.





REFERENCES:
https://mgattozzi.com/diesel-powered-rocket