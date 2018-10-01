// placeholder? WHere image parsing and hashing happens.

use self::models::Photo;
//    PhotoTable (id) {

fn create_photo(conn: &DbConn,
                filename: &String,
                buf : &Vec<u8>
) -> Photo {
    use self::schema::PhotoTable;

    let new_post = Photo {
        title: title,
        body: body,
    };

    diesel::insert_into(PhotoTable::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new Photo")
}
    
