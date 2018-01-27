#[macro_use]
extern crate diesel;
extern crate dotenv;
use self::dotenv::dotenv;

use diesel::prelude::*;
use diesel::sqlite;
use std::env;
//pub mod schema;
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
