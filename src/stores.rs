pub trait CRUD {
    // All object stores in this framework should implement these.

    // The Option<> is to distinbuish between a PUT and a POST.
    // for filesystem stores, use mktemp??
    fn create(&self,path:Option<String>)->Result<String> ;
    fn read(&self,path:String)->Result<String> ;

    // THis will need some refinement. 
    fn update(&self,path:String)->Result<String> ;
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
