use rocket;
use std::io;


pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    rocket.mount(base_address, routes![get_static_files])
}



#[get("/last-home-ping")]
fn get_static_files() -> io::Result<String> {
    Ok(get_latest_ping_time().to_string())
}

fn get_latest_ping_time() -> u64
{
    //ZZZ TODO Write a nice function that reads from my file.
    1493062433   //2017-04-24T19:33:53+00:00
}
