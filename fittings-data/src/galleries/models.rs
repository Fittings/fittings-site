use database::schema::{galleries, gallery_images};


#[derive(Queryable, Identifiable)]
#[has_many(gallery_images)]
#[table_name = "galleries"]
pub struct Gallery {
    pub id: i32,
    pub name: String,
    pub description: String,
}
//
//#[derive(Queryable, Identifiable)]
//#[belongs_to(image_locations, foreign_key="image_id")]
//#[table_name = "gallery_images"]
//pub struct GalleryImages {
//    pub id: i32,
//    pub image_id: i32,
//}
