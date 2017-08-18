use rocket::Rocket;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request};
use rocket_contrib::Json;
use rocket::outcome::IntoOutcome;
use rocket::http::Status;
use fittings_data::users;
use security;
use security::HashResult;
use time::Duration;



pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![authenticate, verify_session, create_user, generate_salt])
}



#[derive(Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
}

#[post("/authenticate", data = "<credentials_json>")]
fn authenticate(mut cookies: Cookies, credentials_json: Json<Credentials>) -> Result<(), Status> {
    let Credentials{username, password} = credentials_json.into_inner();
    println!("attempting to authenticate: {}", &username); //ZZZ TODO Store attempts to DB

    if let Some(user) = users::get_stored_user_data(&username) {
        if security::is_same_hash(&password, &user.hash, &user.salt) {
            let cookie: Cookie = Cookie::build("username", username)
                .http_only(true)
                .max_age(Duration::minutes(2))
                .finish();
            cookies.add_private(cookie);

            return Ok(())
        }
    }

    Err(Status::Unauthorized)
}

pub struct RocketUser {
    pub id: i32,
    pub username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for RocketUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RocketUser, ()> {
        request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|username| users::get_stored_user_data(&username).unwrap())
            .map(|user|  RocketUser {id: user.id, username: user.username})
            .or_forward(())
    }
}


#[get("/verify_session")]
fn verify_session(_user: RocketUser) -> Result<(), Status> {
    Ok(())
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
