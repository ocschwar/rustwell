#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;
//extern crate time;
extern crate diesel;
//extern crate sqlite;
use docopt::Docopt;
use std::str;
use std::env::args;
mod lib;
use lib::establish_connection;
//use time::Timespec;

//use diesel::prelude::*;
//use diesel::sqlite;
/*use sqlite3::{
    DatabaseConnection,
    Query,
    ResultRow,
    ResultRowAccess,
    SqliteResult,
    StatementUpdate,
};*/

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

    let args:Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    let connection = establish_connection();
/*    for db in args.arg_resource {

        let connection = SqliteConnection::establish(&db).unwrap();
        println!("{:?}",db);
        let qr = connection.execute("SELECT id, filename FROM PhotoTable;").unwrap();
        for (name, age) in qr.iter() {
            println!("{} {}", name, age); 
        }
    }
    // TODO: read each resource, dump some contents..
*/
}
