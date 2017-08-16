#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate names;
extern crate fittings_data;
extern crate rocket;
extern crate rocket_contrib;
extern crate rand;
extern crate time;
extern crate argon2rs;
extern crate dotenv;

pub mod security;
pub mod standard;
pub mod uptime;

