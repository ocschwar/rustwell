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
