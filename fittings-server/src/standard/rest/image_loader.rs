use rocket::{Data, Rocket};
use std::io::Read;
use std::fs::File;
use names::Generator;
use fittings_data::images;
use rocket::http::Status;
use standard::rest::authentication::RocketUser;


/// Mounts all the image loading REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![load_image, upload_image])
}


/// Loads the image based on the unique upload image id it was given.
#[get("/image/<image_name>", rank = 1)]
fn load_image(image_name: String) -> Option<File> {
    images::load_image(image_name)
}


/// Uploads an image.
/// Returns the identifier which can be used to retrieve the image from get_dynamic_image(...)
#[post("/image/upload", format = "image/png", data = "<image>")]
fn upload_image(user: RocketUser, image: Data) -> Result<String, Status> {
    if user.username != "cmilsom" { return Err(Status::InternalServerError); }

    let mut image_bytes: Vec<u8> = Vec::new();

    if let Ok(_) = image.open().read_to_end(&mut image_bytes) {
        let mut generator = Generator::default();
        let name = generator.next().unwrap();

        if let Ok(image_path) = images::store_image(name.clone(), image_bytes) {
            let _ = images::insert_image_location(name, image_path.clone()); //ZZZ TODO Do some pattern matching to handle the error. Will need to update the return type.
            return Ok(image_path);
        }
    }

    return Err(Status::InternalServerError);
}
