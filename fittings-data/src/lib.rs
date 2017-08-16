#![feature(plugin)]
#![feature(custom_derive)]
#![feature(custom_attribute)]



#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;



pub mod database;
pub mod images;
pub mod galleries;
pub mod users;
