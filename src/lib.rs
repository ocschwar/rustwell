#![recursion_limit="4096"]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate failure;
#[macro_use] extern crate failure_derive;
use self::dotenv::dotenv;
use diesel::prelude::*;
use diesel::sqlite;
use std::env;
pub mod schema;
pub mod models;
pub mod stores;
use diesel::sqlite::SqliteConnection;

// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
//#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
pub use errors::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
//}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
