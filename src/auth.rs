/*
This is borrowed from https://gitlab.com/C0balt/oxidized-cms
*/

// imports
use crate::api::errors::APIError;
use crate::config::AuthenticationConfig;
use crate::db::model::SlimUser;
use crate::frontend::errors::UserError;
use sodiumoxide::crypto::pwhash::argon2id13;
use sodiumoxide::init;
use sodiumoxide::randombytes::randombytes_into;
use std::fs::File;
use std::io::{Error as IOError, Write};
use std::path::Path;

pub fn generate_key(config: &AuthenticationConfig) -> Result<[u8; 4096], IOError> {
    // init crypto library to auto seed generators etc.
    match init() {
        Ok(_) => (),
        Err(_) => panic!("sodiumoxide couldn't be initialized"),
    }

    // create buffer and fill with random data
    let mut key_buffer: [u8; 4096] = [0; 4096];
    randombytes_into(&mut key_buffer);

    // create key file according to ApplicationConfig Policy
    let path = Path::new(&config.file);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `key` string to `file`, returns `io::Result<()>`
    match file.write_all(&key_buffer) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    Ok(key_buffer)
}

/*
Password hashing and comparison with sodiumoxide
Based on https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/
The article is a wonderful writeup and even covers postgresql specifically
We need modest-strong crypto to protect against e.g. timing attacks as out service is built to be public

These functions don't require sodiumoxide::init due to init being called on thread creation (see app factory closure in main.rs)
*/

pub fn verify_hash(hash: &String, passwd: &str) -> bool {
    // taking hash and converting to (padded) u8 slice
    let mut padded = [0u8; 128];
    hash.as_bytes().iter().enumerate().for_each(|(i, val)| {
        padded[i] = val.clone();
    });
    match argon2id13::HashedPassword::from_slice(&padded) {
        Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}

pub fn generate_hash(password: String) -> String {
    let hash = argon2id13::pwhash(
        password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    std::str::from_utf8(&hash.0).unwrap().to_string()
}

// authentication guard wrapper function
pub fn guard_user(id: &Option<SlimUser>) -> Result<(), UserError> {
    match id {
        Some(_) => Ok(()),
        None => Err(UserError::AuthorizationError {}),
    }
}

pub fn guard_with_user(id: Option<SlimUser>) -> Result<SlimUser, UserError> {
    match id {
        Some(identity) => Ok(identity),
        None => Err(UserError::AuthorizationError {}),
    }
}

pub fn guard_api(id: &Option<SlimUser>) -> Result<(), APIError> {
    match id {
        Some(_) => Ok(()),
        None => Err(APIError::AuthorizationError {}),
    }
}
