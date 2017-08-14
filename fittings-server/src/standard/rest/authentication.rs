use rocket::{Data, Rocket};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;

use time::Duration;



pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![authenticate, verify_authentication, create_user])
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
fn create_user(user_json: Json<CreateUser>) {
    let create_user = user_json.into_inner();

    println!("username: {}", create_user.username);
    println!("password: {}", create_user.password);
}

//*
// * User Authentication & Token Management
// * @see https://github.com/skinkade/rocket-jwt-roles-demo/
// */
//use argon2rs::verifier::Encoded;
//use time;
//
//#[derive(Deserialize)]
//pub struct UserToken {
//    username: String,
//    issue_time: i64,
//    expire_time: i64,
//}
//
//impl UserToken {
//    fn is_expired(&self) -> bool {
//        time::get_time().sec >= self.expire_time
//    }
//}

//
//
//use rocket::Data;
//use rocket::request::Form;
//use rocket::http::{Cookie, Cookies};
//use rocket::response::Redirect;
//
//#[derive(FromForm)]
//struct Message {
//    message: String,
//}
//
//#[post("/cookiez", data = "<message>")]
//fn submit(mut cookies: Cookies, message: Form<Message>) -> Redirect {
//    cookies.add(Cookie::new("message", "some message!"));
//    Redirect::to("/cookie")
//}
//
//#[get("/cookie")]
//fn index(cookies: Cookies) {
//    let cookie = cookies.get("message");
//
//    match cookie {
//        Some(ref val) => println!("Cookie!"),
//        None => println!("No cookie..."),
//    };
//
//}