#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate fittings_server;

use fittings_server::examples;
use fittings_server::standard;
use fittings_server::uptime;


fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();

    let rocket = examples::mount(rocket, "/");
    let rocket = standard::mount(rocket, "/");
    let rocket = uptime::mount(rocket, "/");

    rocket
}
