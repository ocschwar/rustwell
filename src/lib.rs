#![recursion_limit="4096"]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
use self::dotenv::dotenv;
use diesel::prelude::*;
use diesel::sqlite;
use std::env;
pub mod schema;
pub mod models;
pub mod stores;
use diesel::sqlite::SqliteConnection;

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
