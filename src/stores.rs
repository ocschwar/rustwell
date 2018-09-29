
extern crate failure;
use failure::{Error, ResultExt};

use std::io;

pub trait CRUD {
    // All object stores in this framework should implement these.

    // TODO: CHeck Anterofit.
    //
    // Using RLIDWKA as a shout out to the AFS community.
    // https://www.cs.cmu.edu/~help/afs/afs_acls.html
    // The Option<> is to distinbuish between a PUT and a POST.
    // for filesystem stores, use mktemp??
    // i
//    fn create(&self,path:String)->Result<String,&str> ;
    // r
    fn read(&self,path:String)->Result<String,io::Error> ;
    // Obtain object meta data, listing a
    // directory or a single resource
    // l
    fn list(&self,path:String)->Result<Vec<String>,io::Error> ;
    //
//    fn write(&self,path:String, content:String) -> Result<String,&str>;
    // a - update resource meta data
//    fn update(&self,path:String)->Result<String,&str> ;
    // d 
//    fn delete(&self,path:String)->Result<String,&str> ;

}

/*
pub enum ObjectStore {
    LocalStore,
    CifsShare,
    RemovableStore

};*/
// TO change to a type class at a later time.

pub struct LocalStore {
    mount_path: String,    
}


impl CRUD for LocalStore {
    // LIST a directory
    // LIST a file
    fn list(&self,path:String)->Result<Vec<String>,io::Error> {
        use std::fs;
        use std::error::Error;
        use errors::ResultExt;
        let paths = fs::read_dir(path);
        //.chain_err(|| "unable to open contacts file")?;;
        match  paths {
            Ok(ls) => Ok(ls.map(|res|
                                res.unwrap().
                                path().
                                as_os_str().to_str()
                                .unwrap().to_string()).collect()),
            //            paths.sort();
            Err(e) => Err(e)//Err(e)//.description())
        
        }
    }
    //

    // open a file for reading.
    fn read(&self, path:String) ->Result<String,io::Error> {
        /* Going from a pathbuf to a string to generalize 
        the trait.
         */ 
        use std::fs::File;
        use std::io::prelude::*;
        use std::error::Error;
        use std::path::{Path, PathBuf};
        let mpath = Path::new(&self.mount_path);
        let jpath = mpath
            .join(&path);
        let fpath=jpath
            .as_path();
        let display = fpath.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&fpath) {
            // The `description` method of `io::Error` returns a string that
            // describes the error (CHECK SYNTAX HERE)
            Err(why) => return Err(why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => Err(why),
            Ok(_) => Ok(s)
        }
        
    }
    
    

}
pub struct CifsShare {

}

pub struct RemovableStore {

}
