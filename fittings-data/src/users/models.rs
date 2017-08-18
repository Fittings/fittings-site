use database::schema::users;


/// Stored user data on the database.
/// IMPORTANT: This should never be visible through the interface.
#[derive(Identifiable, Queryable, Associations)]
#[primary_key(id)]
#[table_name = "users"]
pub struct StoredUser {
    pub id: i32,
    pub username: String,
    pub hash: String,
    pub salt: String,
}



/// New user data to be stored on the server.
/// The hash and salt should be generated from the fittings_server::security module.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hash: String,
    pub salt: String,
}