use rocket::Rocket;

mod rest {
    pub mod static_files;
    pub mod image_loader;
}


/// Mounts all the standard REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    let address = [base_address, ""].concat(); //Standard mounts on the base address

    let rocket = rest::static_files::mount(rocket, address.as_str());
    let rocket = rest::image_loader::mount(rocket, address.as_str());

    rocket
}
