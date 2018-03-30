pub trait CRUD {
    // All object stores in this framework should implement these.

    // TODO: CHeck Anterofit.
    //
    // Using RLIDWKA as a shout out to the AFS community.
    // https://www.cs.cmu.edu/~help/afs/afs_acls.html
    // The Option<> is to distinbuish between a PUT and a POST.
    // for filesystem stores, use mktemp??
    // i
    fn create(&self,path:String)->Result<String,Error> ;
    // r
    fn read(&self,path:String)->Result<String,Error> ;
    // Obtain object meta data, listing a
    // directory or a single resource
    // l
    fn list(&self,path:String)->Result<Vec<String>,Error> ;
    //
    fn write(&self,path:String, content:String) -> Result<String,Error>;
    // a - update resource meta data
    fn update(&self,path:String)->Result<String,Error> ;
    // d 
    fn delete(&self,path:String)->Result<String,Error> ;

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
    fn list(&self,path:String)->Result<Vec<String>,Error> {
        use std::fs;
        let paths = fs::read_dir(path);
        paths
    }
    //

    // open a file for reading.
    fn read(&self, path:String) ->Result<String,Error> {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::{Path, PathBuf};
        let fpath = Path::new(self.mount_path)
            .join(path)
            .as_path();
        let display = fpath.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&fpath) {
            // The `description` method of `io::Error` returns a string that
            // describes the error (CHECK SYNTAX HERE)
            Err(why) => return format!("couldn't open {}: {}", display,
                                       why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => format!("couldn't read {}: {}", display,
                                why.description()),
            Ok(_) => _
        }
        
    }
    
    

}
pub struct CifsShare {

}

pub struct RemovableStore {

}
