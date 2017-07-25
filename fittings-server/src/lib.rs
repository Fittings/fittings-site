#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate names;
extern crate fittings_data;
extern crate rocket;
extern crate rocket_contrib;
extern crate rand;
extern crate argon2rs;
extern crate time;

pub mod examples;
pub mod standard;
pub mod uptime;

