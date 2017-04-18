use rocket;
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


pub fn mount(rocket: rocket::Rocket, base_address: &str) -> rocket::Rocket {
    rocket.mount(base_address, routes![get_static_files, index])
}



#[get("/<file..>")]
fn get_static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("./").join(file);
    NamedFile::open(path).ok()
}

//ZZZ TODO This needs to be setup correctly. But I need to wait on a particular changes.
// #[get("/<file..>?<_anything>")]
// fn get_static_files_param(file: PathBuf, _anything: Ignored) -> Option<NamedFile> {
//     get_static_files(file)
// }


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("./index.html")
}



//Structure used for ignoring URL params. See: {https://github.com/SergioBenitez/Rocket/issues/219#issuecomment-285257085}
struct Ignored;

impl<'f> rocket::request::FromForm<'f> for Ignored {
    type Error = ();

    fn from_form_items(_items: &mut rocket::request::FormItems<'f>) -> Result<Self, Self::Error> {
        Ok(Ignored)
    }
}
