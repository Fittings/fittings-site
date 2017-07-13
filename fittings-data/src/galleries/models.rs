use database::schema::{galleries, gallery_images};



#[derive(Identifiable, Queryable, Associations)]
#[has_many(gallery_images, foreign_key="gallery_id")]
#[table_name = "galleries"]
pub struct Gallery {
    pub id: i32,
    pub name: String,
    pub description: String,
}


#[derive(Insertable)]
#[table_name = "galleries"]
pub struct NewGallery {
    pub name: String,
    pub description: String,
}



#[derive(Identifiable, Queryable, Associations)]
#[primary_key(gallery_id, image_id)]
#[derive(Insertable)]
#[table_name = "gallery_images"]
#[belongs_to(Gallery)]
pub struct GalleryImage {
    pub gallery_id: i32,
    pub image_id: i32,
}
