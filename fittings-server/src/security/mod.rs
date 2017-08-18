use rand;
use rand::Rng;
use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};
use argon2rs::defaults::{KIB, LANES, PASSES};



//Note: This module is derived from:
//      https://elliotekj.com/2017/04/02/hashing-sensitive-data-in-rust-with-argon2rs/

pub struct HashResult {
    pub hash: String,
    pub salt: String,
}

pub fn hash_data(data: &String) -> HashResult {
    let salt = generate_random_salt();
    let hash = hash(data, &salt);

    HashResult { hash: hash, salt: salt }
}

pub fn is_same_hash(data: &String, hash: &String, salt: &String) -> bool {
    hash == &self::hash(data, salt)
}

fn hash(data: &String, salt: &String) -> String {
    //initialize argon
    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();

    //Create a hash array from our data and salt
    let hash = Encoded::new(a2, data.as_bytes(), salt.as_bytes(), b"", b"").to_u8();

    //encode into a safer string encoding and return
    String::from_utf8(hash).unwrap()
}


pub fn generate_random_salt() -> String {
    rand::thread_rng().gen_ascii_chars().take(32).collect::<String>()
}