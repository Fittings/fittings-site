extern crate rocket;

mod servers
{
    pub mod paste_bin;
}


pub fn mount_examples(rocket: &rocket::Rocket, base_address: &str) {
    println!("Mounting examples on: {}", base_address);

    //let routes = servers::paste_bin::get_routes().iter_mut().map(|route| { route.set_path(base_address)});
}
