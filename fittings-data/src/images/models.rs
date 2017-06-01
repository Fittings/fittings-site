use database::schema::image_locations;


#[derive(Queryable)]
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
