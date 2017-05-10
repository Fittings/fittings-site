#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate rocket_contrib;
extern crate rand;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod examples;
pub mod standard;
pub mod uptime;
pub mod database;
