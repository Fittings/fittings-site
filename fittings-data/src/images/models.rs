use database::schema::{image_locations, gallery_images};


#[derive(Identifiable, Queryable, Associations)]
#[primary_key(id)]
#[table_name = "image_locations"]
#[belongs_to(GalleryImage)]
#[has_many(gallery_images, foreign_key="image_id")]
pub struct ImageLocation {
    pub id: i32,
    pub name: String,
    pub url: String,
}


#[derive(Insertable)]
#[table_name="image_locations"]
pub struct SubmitImageLocation {
    pub name: String,
    pub url: String,
}
