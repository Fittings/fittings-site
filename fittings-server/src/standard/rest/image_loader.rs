use rocket::Rocket;
use database;
use std::path::Path;
use diesel::prelude::*;
use rocket::response::NamedFile;



/// Mounts all the image loading REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![get_dynamic_image])
}



#[get("/image/<image_id>", rank=1)]
fn get_dynamic_image(image_id: i32) -> Option<NamedFile> {
//    ["./image/", image_id.to_string()].concat()
    let path = format!("./image/{}", image_id);
    NamedFile::open(Path::new(path.as_str())).ok()

//    use database::schema::image_locations::dsl::*;
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


}
