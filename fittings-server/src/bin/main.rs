#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate fittings_server;

use fittings_server::examples;

fn main() {
    let fittings_rocket = rocket::ignite();

    let routes = examples::get_routes("/");//(fittings_rocket, "/");

    fittings_rocket.mount("/", routes).launch();
    // rocket::ignite().launch()
}
