#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;
//extern crate time;
extern crate diesel;
extern crate rustwell;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;
extern crate rocket_contrib;
use self::dotenv::dotenv;
use docopt::Docopt;
use std::str;
use std::env;
use rustwell::*;
use self::models::*;
use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
//use rocket::response::content::Json;
use rocket_contrib::Json;
//use rocket
// An alias to the type for a pool of Diesel SQLite connections.
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;



/// Initializes a database pool.
fn init_pool() -> Pool {
    dotenv().ok();
    // The URL to the database, set via the `DATABASE_URL` environment variable.
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("db pool")
}

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[get("/photo")]
fn list_photos(conn: DbConn) -> Json<Vec<Photo>> {
    use self::schema::PhotoTable::dsl::*;
    let results = PhotoTable
        .load::<Photo>(&*conn)          
        .expect("Error loading PhotoTable");
    Json(results)
}
#[get("/photo/<ID>")]
fn get_photo(conn:DbConn, ID:i32) -> Json<Photo> {
    use self::schema::PhotoTable::dsl::*;
    let result = PhotoTable
        .find(ID).first::<Photo>(&*conn)
        .expect("Error loading PhotoTable");
    Json(result)
}

//    rocket::ignite().mount("/", routes![index]).launch();

const USAGE: &'static str = "
Usage: slave [options] [<resource>] ...

Options:
    --addr=<addr>  # Base URL  [default: 127.0.0.1:502].
";
#[derive(Debug, Deserialize)]
struct Args {
    arg_resource: Vec<String>,
    flag_addr: String
}


fn main() {
    rocket::ignite()
        .mount("/", routes![list_photos,get_photo])
        .manage(init_pool())
        .launch();
    // TODO: read each resource, dump some contents..
}
