use dotenv::dotenv;
use std::env;
use rand;
use rand::Rng;
use argon2rs;
use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};
use argon2rs::defaults::{KIB, LANES, PASSES};



//Note: This module is derived from:
//      https://elliotekj.com/2017/04/02/hashing-sensitive-data-in-rust-with-argon2rs/



pub fn hash_data(data: String) -> String {
    //generate a random salt and hash it.
    //ZZZ TODO Investigate this more, I don't think we need to hash our salt like elliotekj suggests.
    let salt = hash(generate_random_salt(), get_local_salt());

    hash(data, salt)
}


fn hash(data: String, salt: String) -> String {
    //initialize argon
    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();

    //Create a hash array from our data and salt
    let hash = Encoded::new(a2, data.as_bytes(), salt.as_bytes(), b"", b"").to_u8();

    //encode into a safer string encoding and return
    String::from_utf8(hash).unwrap()
}


fn get_local_salt() -> String {
    env::var("LOCAL_SALT").expect("LOCAL_SALT must be set.")
}

pub fn generate_random_salt() -> String {
    rand::thread_rng().gen_ascii_chars().take(32).collect::<String>()
}