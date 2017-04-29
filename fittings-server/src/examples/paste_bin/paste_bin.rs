use examples::paste_bin::paste_id::PasteID;
use std::io;
use std::fs::File;
use std::fs;
use std::path::Path;
use rocket;
use rocket::Data;
use rocket::response::content;


const HOST: &'static str = "http://localhost:8000";
const ID_LENGTH: usize = 3;


pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    rocket.mount(base_address, routes![index, retrieve, upload])
}


#[post("/pastebin", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    fs::create_dir_all(Path::new("upload"))?;

    let id = PasteID::new(ID_LENGTH);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = HOST, id = id);

    paste.stream_to_file(Path::new(&filename))?;

    Ok(url)
}

#[get("/pastebin/<id>")]
fn retrieve(id: PasteID) -> Option<content::Plain<File>> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).map(|f| content::Plain(f)).ok()
}

#[get("/pastebin")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXMAPLE: curl --data-binary @file.txt http://localhost:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}
