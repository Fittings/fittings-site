#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate fittings_server;

use fittings_server::examples;

fn main() {
    let fittings_rocket = rocket::ignite();

    examples::mount_examples(&fittings_rocket, "/");
    // rocket::ignite().launch()
}
