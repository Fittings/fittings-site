use rocket::Rocket;

mod rest {
    pub mod static_files;
    pub mod image_loader;
    pub mod galleries;
}


/// Mounts all the standard REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    let address = [base_address, ""].concat(); //Standard mounts on the base address

    let rocket = rest::static_files::mount(rocket, address.as_str());
    let rocket = rest::image_loader::mount(rocket, address.as_str());
    let rocket = rest::galleries::mount(rocket, address.as_str());

    rocket
}


/*
 * User Authentication & Token Management
 * @see https://github.com/skinkade/rocket-jwt-roles-demo/
 */
use argon2rs::verifier::Encoded;
use time;


#[derive(Deserialize)]
struct UserToken {
    username: String,
    issue_time: i64,
    expire_time: i64,
}

impl UserToken {
    fn is_expired(&self) -> bool {
        time::get_time().sec >= self.expire_time
    }
}
