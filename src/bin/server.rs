#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde;
//extern crate docopt;
//extern crate time;
extern crate diesel;
extern crate rustwell;
#[macro_use]
extern crate rocket_contrib;
extern crate rexif;
extern crate jpeg_decoder;
extern crate sha2;

extern crate crypto;
extern crate multipart;
extern crate rustc_serialize;

use rustc_serialize::hex::ToHex;         
use multipart::mock::StdoutTee;
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use rocket::Data;

use rocket::http::{ContentType,Status};
use rocket::response::{Stream,Failure,NamedFile};
use rocket::response::status::{NoContent,Custom,NotFound};

use std::str;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Cursor, Write};
use rustwell::*;
use self::models::*;
use diesel::prelude::*;

use rocket_contrib::{Json};// Value
use sha2::{Sha512,Digest};

use std::ops::Deref;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use jpeg_decoder::Decoder;

extern crate dotenv;
extern crate r2d2_diesel;
extern crate r2d2;

use self::dotenv::dotenv;
use r2d2_diesel::ConnectionManager;


use diesel::sqlite::SqliteConnection;
//use rocket::request::{self, FromRequest};
//use rocket::{Request, State, Outcome};

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
    match query {
        None => (),
        Some(q) => {
            bq = match q.title{
                None=>bq,
                Some(x)=>bq.filter(title.eq(x))
            };
            bq = match q.re_title{
                None=>bq,
                Some(x)=>bq.filter(title.like(x))
            };
            bq = match q.developer{
                None=>bq,
                Some(x)=>bq.filter(developer.eq(x))
            };
            bq = match q.filename{
                None=>bq,
                Some(x)=>bq.filter(filename.eq(x))
            };
            bq = match q.md5{
                None=>bq,
                Some(x)=>bq.filter(md5.eq(x))
            };
            bq = match q.comment{
                None=>bq,
                Some(x)=>bq.filter(comment.eq(x))
            };
            bq = match q.created_after{
                None=>bq,
                Some(x)=>bq.filter(time_created.gt(x))
            };
            bq = match q.limit{
                None=>bq,
                Some(x)=>bq.limit(x)
            };
            bq = match q.offset{
                None=>bq,
                Some(x)=>bq.offset(x)
            };
            bq = match q.id{
                None=>bq,
                Some(x)=>bq.filter(id.eq(x))
            };
            bq = match q.created_before{
                None=>bq,
                Some(x)=>bq.filter(time_created.lt(x))
            };
            bq = match q.re_comment{
                None=>bq,
                Some(x)=>bq.filter(comment.like(x))
            };
            bq = match q.transformations{
                None=>bq,
                Some(x)=>bq.filter(transformations.eq(x))
            };
            bq = match q.backlinks{
                None=>bq,
                Some(x)=>bq.filter(backlinks.eq(x))
            };
        }
    }
            
    let results = bq.load::<Photo>(&*conn)          
        .expect("Error loading PhotoTable");    
    Json(results)
}

/*

#[put("/<id>", data = "<photo>")]
fn update_photo(id: i32, photo: Json<Photo>, conn: DbConn) -> Json<Value> {
    use self::schema::PhotoTable::dsl::*;
    let update = Photo { id: id, ..photo.into_inner() };
    Json(json!({
        "success": Photo::update(id, update, &conn)
    }))
}

#[put("/<id>", format = "application/json", data = "<photo>")]
fn put(id: i32, photo: Json<Photo>, connection: DbConn) ->
    std::result::Result<Json<Photo>, Failure> {
    PhotoTable::update(id, photo.into_inner(), &connection)
        .map(|photo| Json(photo))
        .map_err(|error| error_status(error))
}
*/
#[get("/photo/<id>")]
fn get_photo(conn:DbConn, id:i32) ->
    std::result::Result<NamedFile,NotFound<String>>{
    use self::schema::PhotoTable::dsl::*;
    let result = PhotoTable
        .find(id).first::<Photo>(&*conn)
        .expect("Error loading PhotoTable");
    println!("{:?}", &result);
    match rexif::parse_file(&result.filename) {
        Ok(exif) => {
            println!("{} {} exif entries: {}", &result.filename,
                     exif.mime, exif.entries.len());
            
            for entry in &exif.entries {
                //println!("{:?}",entry);
                println!("  {}: {}",
                         entry.tag,
                         entry.value_more_readable);
            }
        },
        Err(e) => {
            print!("Error in {}: {}", &result.filename, e)
        }
    }
        //    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {}", path)))
    let ff = File::open(&result.filename);
    match ff {
        Ok(mut f)=> {
            let mut buffer = Vec::new();

            // read the whole file
            f.read_to_end(&mut buffer).expect("File unread");
            f.seek(std::io::SeekFrom::Start(0));
            let mut decoder = Decoder::new(&f);//BufReader::new(f));
            decoder.read_info().expect("failed to read metadata");
            let metadata = decoder.info().unwrap();
            println!("{:?}",metadata);
            // create a Sha512 object
            let mut hasher = Sha512::default();
        
            // write input message
            hasher.input(&buffer);
            
            // read hash digest and consume hasher
            let output = hasher.result();
            println!("HASH {:?}",&output);
            Ok( NamedFile::open(&result.filename).unwrap())
        },
        
        Err(e)=> {Err(NotFound(format!("404 {}",e)))}
    }
}

#[post("/upload", data = "<data>")]//, format = "multipart/form-data")]
// signature requires the request to have a `Content-Type`
fn multipart_upload(
    conn:DbConn,
    data: Data,
    cont_type:&ContentType) -> std::result::Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
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

    match process_upload(boundary, data ,conn) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data,    conn:DbConn
) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out,conn)?,
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out,conn)?
        },
        multipart::server::save::SaveResult::Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to
fn process_entries(entries: Entries, mut out: &mut Vec<u8>,conn:DbConn) -> io::Result<()> {
    {
        let stdout = io::stdout();
        let tee = StdoutTee::new(&mut out, &stdout);
        for (k,v) in &entries.fields {
            println!("{} {:?}",k,v);
            for f in v{
                println!("{:?}",f);                                
                let fname=match &f.headers.filename {
                    Some(x) => { //let fname = x;
                        println!("FName {}",x);
                        x
                    },
                    None => {
                        println!("NoName");
                        ""
                    }
                };
                println!("{:?}",f.data.size());
                let mut r = f.data.readable();
                match r {
                    Ok(mut R) => {
                        println!("readable ");
                        let mut ibuffer = Vec::new();
                        // read the whole file
                                     
                        R.read_to_end(&mut ibuffer).expect("File unread");
                        create_photo(&conn,
                                     fname,
                                     &ibuffer);
                        let mut hasher = Sha512::new();

                        // write input message
                        hasher.input(&ibuffer);
                        
                        // read hash digest and consume hasher
                        let output = hasher.result().to_hex();
                        println!("HASH {:?}",&output);
                        match rexif::parse_buffer(&ibuffer) {
                            Ok(exif) => {
                                println!("EXIF {:?}",&exif);
                            },
                            Err(e) =>{
                                println!("noexif {:?}",&e);
                            }
                        }

                    },
                    Err(x)=>println!("not readable {:?}",x)
                }

            }
            
        }

    }
    
    writeln!(out, "Entries processed@!!")
}

#[delete("/photo/<id>")]
fn delete_photo(
    id: i32, connection: DbConn
) ->std::result::Result<NoContent, Failure> {
    use self::schema::PhotoTable;//::dsl::*;
    let del = PhotoTable::table.find(id);
    println!("{:?}",del);
    let f = del
        .first::<Photo>(&*connection);
    // THis is needed becayse this table query otherwise
    // causes the first photo to be returned. 
    println!("{:?}",f);
    match f {
        Ok(photo) => {
            println!("{:?}",photo);
            match photo.id {
                id => match diesel::delete(&photo).execute(&*connection){
                    Ok(_)=> Ok(NoContent),
                    Err(_)=>Err(Failure (Status::InternalServerError))
                },
                _ =>Err(Failure (Status::InternalServerError))
                    
            }
        },
        Err(e) => Err(Failure (Status::InternalServerError))
    }
}


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

fn create_photo(conn: &DbConn,
                fname: & str,
                buf : & Vec<u8>
) ->usize {

    use self::schema::PhotoTable::dsl::*;
    use std::io::Cursor;

    let curs = Cursor::new(buf);

    let mut  new_photo = NewPhoto {
        id:None,
        filename:fname.to_string() ,
        filesize:Some(buf.len() as i32),
        /* 
        timestamp - file time stamp
        import_id - current time
        event_id ?? 

*/
        ..Default::default()
    };
    let mut decoder = Decoder::new(curs);//BufReader::new(f));
    decoder.read_info().expect("failed to read metadata");
    let metadata = decoder.info();
    match metadata {
        Some(m) => {
            new_photo.width = Some(m.width as i32);
            new_photo.height = Some(m.height as i32);

            /*
"exposure_time":1500212364, - EXIF
"orientation":1, EXIF
"original_orientation":1,
"transformations":null,
"md5":"eca63be5041cd3744036987992c850f2",
"thumbnail_md5":null,
"time_created":1500216692,
"exif_md5":"fa4a491a922e78c7b631c80353d64f60",
"flags":0,
"rating":0,
"file_format":0,
"title":null,
"backlinks":null,
"time_reimported":null,
"editable_id":-1,
"metadata_dirty":0,
"developer":"SHOTWELL",
"develop_shotwell_id":-1,
"develop_camera_id":-1,
"develop_embedded_id":-1,
"comment":null},*/
        },
        None => {
            println!("nojpeg ");
        }
    }
    println!("{:?}",metadata);
    match rexif::parse_buffer(&buf) {
        Ok(exif) => {
            println!("EXIF {:?}",&exif);
        },
        Err(e) =>{
            println!("noexif {:?}",&e);
        }
    }
//    let create_query: QueryResult<Photo> =
    println!("{:?}",new_photo);
        diesel::insert_into(PhotoTable)
            .values(&new_photo)
            .execute(&**conn)//
            //.get_result(&**conn)
        .expect("Error saving new Photo")
    
}


fn main() {
    rocket::ignite()
        .mount("/", routes![list_photos,get_photo,delete_photo,
                            list_some_photos,multipart_upload])
        .manage(init_pool())
        .launch();
    // TODO: read each resource, dump some contents..
}
