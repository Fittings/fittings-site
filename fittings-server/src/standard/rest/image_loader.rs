use rocket::{Data, Rocket};
use std::io;
use std::io::Read;
use std::fs::File;
use names::Generator;
use fittings_data::images;






/// Mounts all the image loading REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![load_image, upload_image])
}



/// Loads the image based on the unique upload image id it was given.
#[get("/image/<image_name>", rank=1)]
fn load_image(image_name: String) -> Option<File> {
    images::load_image(image_name)
}


/// Uploads an image.
/// Returns the identifier which can be used to retrieve the image from get_dynamic_image(...)
#[post("/image/upload", format = "image/png", data = "<image>")]
fn upload_image(image: Data) -> io::Result<String> {
    let mut image_bytes : Vec<u8> = Vec::new();
    image.open().read_to_end(&mut image_bytes)?;

    let mut generator = Generator::default();
    let name = generator.next().unwrap();
    let id : String = images::upload_image(name, image_bytes)?;

    Ok(String::from(id))
}
