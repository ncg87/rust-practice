// bring database.rs file into scope
mod database;
// bring auth_utils.rs folder into scope, since auth_utils is a folder it looks for mod.rs file in the folder
mod auth_utils;

// bring Credentials into scope, each has to be public
pub use auth_utils::models::Credentials; // re-exporting Credentials from auth_utils to upper modules
use database::Status;

use rand::prelude::*; // kleene start to bring all the functions from rand module into scope
use std::thread;
use std::time::Duration;

pub fn authenticate(credentials: Credentials) { // public to the outside world
    let timeout = thread_rng().gen_range(100..500); // using fn from rand module
    println!("Simulating a database timeout of {}ms", timeout);
    thread::sleep(Duration::from_millis(timeout)); // using fn from std::thread module
    if let Status::Success = database::connect_to_database(&credentials) {
        auth_utils::login(credentials)
    }
}

