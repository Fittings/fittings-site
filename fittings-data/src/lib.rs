#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]


#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

pub mod database;
