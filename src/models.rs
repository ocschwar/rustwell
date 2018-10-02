
use schema::{PhotoTable,VideoTable};//,ObjectStore,ObjectLocation};
/*
#[table_name="ObjectLocation"]
#[derive(Queryable,Identifiable,Debug)]
pub struct ObjectLocation {
    pub class:enum{Video,Photo};
    pub id:i32,
    pub path:String,
    pub store: <CRUD>;
}
 */

// TODO: Define ObjectStore for Diesel.
// For both the OL and OS tables

#[table_name="VideoTable"]
#[derive(Queryable,Identifiable,Debug)]
pub struct Video {
    pub id: i32,
    pub filename: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub         clip_duration : Option<f32>,
    pub         is_interpretable : Option<i32>,
    pub         filesize : Option<i32>,
    pub         timestamp : Option<i32>,
    pub         exposure_time : Option<i32>,
    pub         import_id : Option<i32>,
    pub         event_id : Option<i32>,
    pub         md5 : Option<String>,
    pub         time_created : Option<i32>,
    pub         rating : Option<i32>,
    pub         title : Option<String>,
    pub         backlinks : Option<String>,
    pub         time_reimported : Option<i32>,
    pub         flags : Option<i32>,
    pub         comment : Option<String>,
}
#[table_name="PhotoTable"]
#[derive(Queryable,Identifiable,
         Debug,Serialize,Insertable,Default)]
pub struct Photo {
    pub id: i32,
    pub filename: String, // 
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub filesize: Option<i32>,
    pub timestamp: Option<i32>,
    pub exposure_time: Option<i32>,
    pub orientation: Option<i32>,
    pub original_orientation: Option<i32>,
    pub import_id: Option<i32>,
    pub event_id: Option<i32>,
    pub transformations: Option<String>, //
    pub md5 : Option<String>, //
    pub thumbnail_md5 : Option<String>,
    pub time_created : Option<i32>,
    pub exif_md5 : Option<String>,
    pub flags : Option<i32>,
    pub rating : Option<i32>,
    pub file_format : Option<i32>,
    pub title : Option<String>, //
    pub backlinks : Option<String>,//
    pub time_reimported : Option<i32>,
    pub editable_id : Option<i32>,
    pub metadata_dirty : Option<i32>,
    pub developer : Option<String>,
    pub develop_shotwell_id : Option<i32>,
    pub develop_camera_id : Option<i32>,
    pub develop_embedded_id : Option<i32>,
    pub comment : Option<String>,
}
