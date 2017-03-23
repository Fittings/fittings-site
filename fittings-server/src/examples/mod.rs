use rocket;

mod servers
{
    pub mod paste_bin;
    mod paste_id;
}

pub fn rocket(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    let address = [base_address, "examples/"].concat();

    servers::paste_bin::rocket(rocket, address.as_str())
}
