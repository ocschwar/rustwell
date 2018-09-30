#![feature(custom_derive)]
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
extern crate rexif;
extern crate sha2;
extern crate multipart;
use multipart::mock::StdoutTee;
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;

use rocket::Data;
use rocket::http::{ContentType,Status};
use rocket::response::Stream;
use rocket::response::status::Custom;

use self::dotenv::dotenv;
use docopt::Docopt;
use std::str;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Cursor, Write};
use rustwell::*;
use self::models::*;
use diesel::prelude::*;
//use rexif;
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

#[derive(FromForm)]
struct PhotoQuery {
    title:Option<String>,
    re_title:Option<String>,
    created_after:Option<i32>,
    created_before:Option<i32>,
    limit:Option<i64>,
    offset:Option<i64>,
    id:Option<i32>,
    md5:Option<String>,
    filename:Option<String>, 
    comment:Option<String>,
    re_comment:Option<String>,
    transformations:Option<String>,
    backlinks:Option<String>,
    developer:Option<String>
}

#[get("/photo",  format = "application/json")]
fn list_photos(conn: DbConn) -> Json<Vec<Photo>> {
    use self::schema::PhotoTable::dsl::*;
    let results = PhotoTable
        .load::<Photo>(&*conn)
        .expect("Error loading PhotoTable");
     Json(results)
}

#[get("/photo?<query>",  format = "application/json")]
fn list_some_photos(conn: DbConn, query:Option<PhotoQuery>) -> Json<Vec<Photo>> {
    use self::schema::PhotoTable::dsl::*;
    let mut bq = PhotoTable.into_boxed();
    match(query){
        None => (),
        Some(q) => {
            bq = match(q.title){
                None=>bq,
                Some(x)=>bq.filter(title.eq(x))
            };
            bq = match(q.re_title){
                None=>bq,
                Some(x)=>bq.filter(title.like(x))
            };
            bq = match(q.developer){
                None=>bq,
                Some(x)=>bq.filter(developer.eq(x))
            };
            bq = match(q.filename){
                None=>bq,
                Some(x)=>bq.filter(filename.eq(x))
            };
            bq = match(q.md5){
                None=>bq,
                Some(x)=>bq.filter(md5.eq(x))
            };
            bq = match(q.comment){
                None=>bq,
                Some(x)=>bq.filter(comment.eq(x))
            };
            bq = match(q.created_after){
                None=>bq,
                Some(x)=>bq.filter(time_created.gt(x))
            };
            bq = match(q.limit){
                None=>bq,
                Some(x)=>bq.limit(x)
            };
            bq = match(q.offset){
                None=>bq,
                Some(x)=>bq.offset(x)
            };
            bq = match(q.id){
                None=>bq,
                Some(x)=>bq.filter(id.eq(x))
            };
            bq = match(q.created_before){
                None=>bq,
                Some(x)=>bq.filter(time_created.lt(x))
            };
            bq = match(q.re_comment){
                None=>bq,
                Some(x)=>bq.filter(comment.like(x))
            };
            bq = match(q.transformations){
                None=>bq,
                Some(x)=>bq.filter(transformations.eq(x))
            };
            bq = match(q.backlinks){
                None=>bq,
                Some(x)=>bq.filter(backlinks.eq(x))
            };
        }
    }
            
    let results = bq.load::<Photo>(&*conn)          
        .expect("Error loading PhotoTable");
    
//    results = match 
    Json(results)
}
// TODO: get photo contents for local file....
#[get("/photo/<ID>")]
fn get_photo(conn:DbConn, ID:i32) -> Vec<u8> {
    use self::schema::PhotoTable::dsl::*;
    let result = PhotoTable
        .find(ID).first::<Photo>(&*conn)
        .expect("Error loading PhotoTable");
    println!("{:?}", &result.filename);
    let mut f = File::open(&result.filename).expect("file not found");
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).expect("File unread");
    match rexif::parse_file(&result.filename) {
        Ok(exif) => {
            println!("{} {} exif entries: {}", &result.filename,
                     exif.mime, exif.entries.len());
            
            for entry in &exif.entries {
                println!("  {}: {}",
                         entry.tag,
                         entry.value_more_readable);
            }
        },
        Err(e) => {
            print!("Error in {}: {}", &result.filename, e)
        }
    }
    use sha2::{Sha512, Digest};

    // create a Sha512 object
    let mut hasher = Sha512::default();

// write input message
    hasher.input(&buffer);

    // read hash digest and consume hasher
    let output = hasher.result();
    println!("HASH {:?}",&output);

    buffer
}

#[post("/upload", data = "<data>")]//, format = "multipart/form-data")]
// signature requires the request to have a `Content-Type`
fn multipart_upload(data: Data,cont_type:&ContentType) -> std::result::Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
    if !cont_type.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into()
        ));
}    
    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
            || Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into()
            )
        )?;

    match process_upload(boundary, data) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out)?,
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out)?
        },
        multipart::server::save::SaveResult::Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to
fn process_entries(entries: Entries, mut out: &mut Vec<u8>) -> io::Result<()> {
    {
        let stdout = io::stdout();
        let tee = StdoutTee::new(&mut out, &stdout);
        entries.write_debug(tee)?;
    }

    writeln!(out, "Entries processed")
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
        .mount("/", routes![list_photos,get_photo,
                            list_some_photos,multipart_upload])
        .manage(init_pool())
        .launch();
    // TODO: read each resource, dump some contents..
    // dotenv the port #
}
