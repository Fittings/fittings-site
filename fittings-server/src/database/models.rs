#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub image: Vec<u8>,
}
