use rocket::{Data, Rocket};
use rocket_contrib::{JSON, Value};
use fittings_data::galleries;
use rocket::http::Status;



/// Mounts all the image loading REST routes to the rocket instance.
pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![get_galleries, get_gallery_images, upload_gallery, upload_gallery_image])
}

#[derive(Serialize, Deserialize)]
struct Gallery {
    id: i32,
    name: String,
    description: String,
    preview_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Galleries {
    galleries : Vec<Gallery>,
}

/// Returns all the image galleries.
/// This doesn't include the images, images will need to be retrieved in get_gallery_images(...) call
#[get("/gallery/all")]
fn get_galleries() -> Option<JSON<Value>> {
    let galleries = match galleries::get_all_image_galleries() {
        Some(galleries) => galleries,
        None => return None,
    };

    let mut gallery_vals : Vec<Gallery> = Vec::new();

    for gallery in galleries {
        let preview_url = match galleries::get_first_image_in_gallery(gallery.id) {
            Some(img_loc) => Some(img_loc.url),
            None => None,
        };

        let gallery_val = Gallery {
            id : gallery.id,
            name : gallery.name,
            description : gallery.description,
            preview_url: preview_url,
        };

        gallery_vals.push(gallery_val);
    }

    Some(JSON(json!(Galleries {
        galleries : gallery_vals,
    })))
}


#[derive(Serialize, Deserialize)]
struct GalleryImages {
    id : i32,
    galleries : Vec<ImageLocation>,
}

#[derive(Serialize, Deserialize)]
struct ImageLocation {
    id: i32,
    name: String,
    url: String,
}


#[get("/gallery/<gallery_id>")]
fn get_gallery_images(gallery_id : i32) -> Option<JSON<Value>> {
    let images = match galleries::get_images_in_gallery(gallery_id) {
        Some(images) => images,
        None => return None,
    };

    let mut image_vals = Vec::new();
    for image in images {
        let image_val = ImageLocation {
            id : image.id,
            name : image.name,
            url : image.url,
        };
        image_vals.push(image_val);
    }

    Some(JSON(json!(GalleryImages {
        id : gallery_id,
        galleries : image_vals,
    })))
}


#[derive(Serialize)]
struct GalleryUploadResponse {
    id: usize,
}

#[derive(Deserialize)]
struct CreateGallery{
    pub name: String,
    pub description: Option<String>,
}

#[post("/upload/gallery", data="<gallery>")]
fn upload_gallery(gallery: JSON<CreateGallery>) -> Result<JSON<GalleryUploadResponse>, Status> {
    let description = match gallery.0.description {
        Some(desc) => desc,
        None => "".to_string()
    };

    match galleries::create_gallery(gallery.0.name, description) {
        Ok(id) => Ok(JSON(GalleryUploadResponse {id: id})),
        Err(e) => Err(Status::InternalServerError),
    }

}

#[post("/upload/gallery/<gallery_id>/image",  data = "<image>")]
fn upload_gallery_image(gallery_id: String, image: Data) -> Result<JSON<Value>, Status> {
    println!("Attempting to upload gallery image: {}", gallery_id);




    Err(Status::NotImplemented)
}