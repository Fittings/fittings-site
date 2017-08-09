use rocket::Rocket;

mod rest {
    pub mod static_files;
    pub mod image_loader;
    pub mod galleries;
    pub mod authentication;
}


/// Mounts all the standard REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    let rocket = rest::authentication::mount(rocket, base_address);
    let rocket = rest::static_files::mount(rocket, base_address);
    let rocket = rest::image_loader::mount(rocket, base_address);
    let rocket = rest::galleries::mount(rocket, base_address);

    rocket
}
