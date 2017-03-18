

extern crate rocket;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![world]
}





// extern crate rocket;
//
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}
