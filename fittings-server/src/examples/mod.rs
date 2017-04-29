use rocket;

mod paste_bin
{
    pub mod paste_bin;
    mod paste_id;
}

pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    let address = [base_address, "examples/"].concat();

    let rocket = paste_bin::paste_bin::mount(rocket, address.as_str());

    rocket
}
