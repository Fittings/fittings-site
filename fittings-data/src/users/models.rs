use database::schema::users;


#[derive(Identifiable, Queryable, Associations)]
#[primary_key(id)]
#[table_name = "users"]
pub struct StoredUser {
    pub id: i32,
    pub username: String,
    pub hash: String,
    pub salt: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hash: String,
    pub salt: String,
}