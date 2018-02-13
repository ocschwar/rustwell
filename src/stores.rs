pub trait CRUD {
    // All object stores in this framework should implement these.
    //
    // Using RLIDWKA as a shout out to the AFS community.
    // https://www.cs.cmu.edu/~help/afs/afs_acls.html
    // The Option<> is to distinbuish between a PUT and a POST.
    // for filesystem stores, use mktemp??
    // i
    fn create(&self,path:Option<String>)->Result<String> ;
    // r
    fn read(&self,path:String)->Result<String> ;
    // Obtain object meta data, listing a
    // directory or a single resource
    // l
    fn list(&self,path:String)->Result<Vec<String>>> ;
    //
    fn write(&self,path:String, content:String) -> Result<String>;
    // a - update resource meta data
    fn update(&self,path:String)->Result<String> ;
    // d 
    fn delete(&self,path:String)->Result<String> ;

}


pub enum ObjectStore {
    LocalStore,
    CifsShare,
    RemovableStore

};
// TO change to a type class at a later time.

pub struct LocalStore {
    mount_path: String,
    
};

pub struct CifsShare {

};

pub struct RemovableStore {

}
