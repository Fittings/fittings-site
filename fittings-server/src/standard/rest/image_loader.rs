use rocket::{Data, Rocket};
use rocket::response::NamedFile;
use std::io;
use std::io::Read;
use std::path::Path;
use names::Generator;
use fittings_data::images;



/// Mounts all the image loading REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![get_dynamic_image, upload_image])
}



#[get("/image/<image_name>", rank=1)]
fn get_dynamic_image(image_name: String) -> Option<NamedFile> {
    let path = format!("./static/media/{}", image_name);
    NamedFile::open(Path::new(path.as_str())).ok()
}

/// Uploads an image.
/// Returns the identifier which can be used to retriece the image from get_dynamic_image(...)
#[post("/image/upload", format = "image/png", data = "<image>")]
fn upload_image(image: Data) -> io::Result<String> {
    let mut image_bytes : Vec<u8> = Vec::new();
    image.open().read_to_end(&mut image_bytes)?;

    let mut generator = Generator::default();
    let name = generator.next().unwrap();
    let id : String = images::upload_image(name, image_bytes)?;

    Ok(String::from(id))
}
