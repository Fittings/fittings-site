
#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub image: Vec<u8>, //ZZZ TODO These really shouldn't be nullable.
}

use super::schema::images;
#[derive(Insertable)]
#[table_name="images"]
pub struct NewImage {
    pub image: Vec<u8>,
}
