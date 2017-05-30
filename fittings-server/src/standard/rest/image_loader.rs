use rocket::{Data, Rocket};
use rocket::response::NamedFile;
use diesel::prelude::*;
use std::io;
use std::path::Path;
use diesel;
use database;
use database::models::{SubmitImageLocation, ImageLocation};
use database::schema::image_locations::dsl::*;
use names::{Generator, Name};



/// Mounts all the image loading REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![get_dynamic_image, upload_image])
}



#[get("/image/<image_id>", rank=1)]
fn get_dynamic_image(image_id: i32) -> Option<NamedFile> {
//    ["./image/", image_id.to_string()].concat()
    let path = format!("./image/public/{}", image_id);
    NamedFile::open(Path::new(path.as_str())).ok()
}

#[post("/image", format = "image/png", data = "<image>")]
fn upload_image(image: Data) -> io::Result<String> {
    let conn = database::get_db_connection();
    let mut generator = Generator::default();
    let name = generator.next().unwrap();

    let image_location = SubmitImageLocation {
        url: name,
    };

    diesel::insert(&image_location)
        .into(image_locations)
        .execute(&*conn)
        .expect("upload image failed to insert");

    //let image_loc : ImageLocation = image_locations.order(id.desc()).first(&*conn).unwrap();

    image.stream_to_file(Path::new(format!("./static/media/{}", &image_location.url).as_str()))?;

    Ok(image_location.url)
}


//
//    let conn = database::get_db_connection();
//
//    let image_result: database::models::ImageLocation = image_locations.find(1).load(&*conn).unwrap().pop().unwrap();

//    match NamedFile::open("foo.png") {
//        Ok(file) => {
//            return Some(file)
//        },
//        Err(_) => (),
//    }


//
//        let mut inside: Vec<u8> = (*the_image.get(0).unwrap()).image;
//
//        let mut file = File::create("foo.png").unwrap();
//        let mut reader = BufReader::new(file);
//        reader.read(inside.as_mut_slice());
