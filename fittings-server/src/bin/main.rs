#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate fittings_server;

use fittings_server::examples;
use fittings_server::standard;
use fittings_server::uptime;

extern crate diesel;
use self::diesel::prelude::*;
use fittings_server::database;


fn main() {
    use fittings_server::database::schema::images::dsl::*;

    let connection = database::establish_connection();
    let results = images.load::<database::models::Image>(&connection).expect("Error loading images");

    println!("Load all images!");
    for an_image in results {
        println!("{}", an_image.id.unwrap());
    }
    println!("Loaded all images...");

    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();

    let rocket = examples::mount(rocket, "/");
    let rocket = standard::mount(rocket, "/");
    let rocket = uptime::mount(rocket, "/");

    rocket
}
