#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate time; 
use docopt::Docopt;
use std::str;
use std::env::args;
extern crate sqlite3;

use time::Timespec;

use sqlite3::{
    DatabaseConnection,
    Query,
    ResultRow,
    ResultRowAccess,
    SqliteResult,
    StatementUpdate,
};

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

    // TODO: read each resource, dump some contents..
}
