use rocket;

mod home
{
    pub mod home;
}

pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    let address = [base_address, ""].concat(); //Standard mounts on the base address

    let rocket = home::home::mount(rocket, address.as_str());

    rocket
}
