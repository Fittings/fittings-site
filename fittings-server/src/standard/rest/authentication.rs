use rocket::{Data, Rocket};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;



pub fn mount(rocket: Rocket, base_address: &str) -> Rocket {
    rocket.mount(base_address, routes![authenticate])
}



#[derive(Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
}

#[post("/authenticate", data = "<credentials>")]
fn authenticate(mut cookies: Cookies, credentials: Json<Credentials>) {
    let cred = credentials.into_inner();
    if cred.username == "cmilsom" && cred.password == "pa55w0rd" {
        println!("{} has been authenticated", cred.username);
        cookies.add_private(Cookie::new("user_id", "1"));
    }
    println!("username: {}", cred.username);
    println!("password: {}", cred.password);
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