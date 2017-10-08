use rocket::Rocket;

mod rest {
    pub mod static_files;
    pub mod image_loader;
    pub mod galleries;
    pub mod authentication;
}


/// Mounts all the standard REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    //Static data loader
    let rocket = rest::static_files::mount(rocket, base_address);

    //REST API
    let api_address = [base_address, "api/"].concat();
    let rocket = rest::authentication::mount(rocket, api_address.as_str());
    let rocket = rest::image_loader::mount(rocket, api_address.as_str());
    let rocket = rest::galleries::mount(rocket, api_address.as_str());

    rocket
}
