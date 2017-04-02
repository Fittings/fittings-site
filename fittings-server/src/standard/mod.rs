use rocket;

mod static_files
{
    pub mod static_files;
}

pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    let address = [base_address, ""].concat(); //Standard mounts on the base address

    let rocket = static_files::static_files::mount(rocket, address.as_str());

    rocket
}