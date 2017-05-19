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

Sources: 
* ***killercup***: [diesel issue #555](https://github.com/diesel-rs/diesel/issues/555#issuecomment-269482063)
* ***mgattozzi***: [diesel-powered-rocket](https://mgattozzi.com/diesel-powered-rocket)



# Client-Side

## ReactJS Setup.
1. Install Yarn:
```bash
curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list

sudo apt-get update && sudo apt-get install yarn
```

2. Install NPM 
```bash
sudo apt-get update
sudo apt-get install nodejs

sudo apt-get install npm
```

  * Fix [/usr/bin/env: node: No such file or directory](http://javascript.tutorialhorizon.com/2015/03/01/troubleshooting-usrbinenv-node-no-such-file-or-directory/) error that will occur in future ```yarn build``` call.
      ```bash
      sudo ln –s /usr/bin/nodejs /usr/bin/node
      ```

# Reverse Proxy and HTTPS setup.





