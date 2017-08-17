use rocket::Rocket;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;
use rocket::http::Status;
use fittings_data::users;
use security;
use security::HashResult;
use time::Duration;



pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![authenticate, verify_authentication, create_user, generate_salt])
}



#[derive(Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
}

#[post("/authenticate", data = "<credentials_json>")]
fn authenticate(mut cookies: Cookies, credentials_json: Json<Credentials>) {
    let cred = credentials_json.into_inner();
    println!("username: {}", &cred.username);
    println!("password: {}", &cred.password);

    if cred.username == "cmilsom" && cred.password == "pa55w0rd" {
        println!("{} has been authenticated", &cred.username);
        let cookie: Cookie = Cookie::build("username", cred.username)
            .http_only(true)
            .max_age(Duration::minutes(2))
            .finish();
        cookies.add_private(cookie);
    }
}

#[get("/verify_authentication")]
fn verify_authentication(mut cookies: Cookies) {
    let stored_username = cookies.get_private("username");
    match stored_username {
        Some(username) => println!("username: {}", username),
        None => println!("no existing session"),
    }
}

#[derive(Deserialize)]
struct CreateUser {
    pub username: String,
    pub password: String,
}


#[post("/create_user", data = "<user_json>")]
fn create_user(user_json: Json<CreateUser>) -> Result<(), Status> {
    let CreateUser {username, password} = user_json.into_inner();
    let HashResult{hash, salt} = security::hash_data(&password);

    match users::create_user(username, hash, salt) {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::InternalServerError),
    }
}

///Utility method
#[get("/generate_salt")]
fn generate_salt() -> String {
    security::generate_random_salt()
}
