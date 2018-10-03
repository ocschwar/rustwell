# rustwell
Rust photo manager inspired by Gnome Shotwell, and written to use the Shotwell database for its back end. 

Uses Rocket and Diesel. 


Note: this is a side project by a father of 3, with limited time and much dad-brain. Expect progress to be slow. 


1. More built out services.

Use THis as a template:
http://docs.imbo-project.org/en/latest/usage/api.html#images-resource-users-user-images

added ?id==ID to the listing to filter by ID (DONE)

Getting the particular ID should get the image itself.
(Using the object table later)

(Now do pagination - add a .offset() clause)

   It takes multiparts now.
   Can save the images. Can calc hashes and parse EXIF from POSTS
      

- SHA hashes

TODO: enter posts into Diesel.
  -  will require moving things out of server.rs at some point. 
  - needs md5 and exif md5
  - autoincrement ids.
result_str for hashes.

TODO: object stores.

TODO: REST Client

TODO: Directory search


---- details for testing:

omri@omri-Satellite-S55-B:~$ curl -F data=\@Downloads/1980s-hairstyles-for-men-1.jpeg http://localhost:8000/upload
 1360  curl http://localhost:8000/photo\?re_title=foo
 1361  curl http://localhost:8000/photo\?re_comment=foo
 1362  curl http://localhost:8000/photo
 1783  curl http://localhost:8000/photo/1

CREATE TABLE oldPhotoTable (id INTEGER PRIMARY KEY, filename TEXT UNIQUE NOT NULL, width INTEGER, height INTEGER, filesize INTEGER, timestamp INTEGER, exposure_time INTEGER, orientation INTEGER, original_orientation INTEGER, import_id INTEGER, event_id INTEGER, transformations TEXT, md5 TEXT, thumbnail_md5 TEXT, exif_md5 TEXT, time_created INTEGER, flags INTEGER DEFAULT 0, rating INTEGER DEFAULT 0, file_format INTEGER DEFAULT 0, title TEXT, backlinks TEXT, time_reimported INTEGER, editable_id INTEGER DEFAULT -1, metadata_dirty INTEGER DEFAULT 0, developer TEXT, develop_shotwell_id INTEGER DEFAULT -1, develop_camera_id INTEGER DEFAULT -1, develop_embedded_id INTEGER DEFAULT -1, comment TEXT);
CREATE TABLE PhotoTable (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, filename TEXT UNIQUE NOT NULL, width INTEGER, height INTEGER, filesize INTEGER, timestamp INTEGER, exposure_time INTEGER, orientation INTEGER, original_orientation INTEGER, import_id INTEGER, event_id INTEGER, transformations TEXT, md5 TEXT, thumbnail_md5 TEXT, exif_md5 TEXT, time_created INTEGER, flags INTEGER DEFAULT 0, rating INTEGER DEFAULT 0, file_format INTEGER DEFAULT 0, title TEXT, backlinks TEXT, time_reimported INTEGER, editable_id INTEGER DEFAULT -1, metadata_dirty INTEGER DEFAULT 0, developer TEXT, develop_shotwell_id INTEGER DEFAULT -1, develop_camera_id INTEGER DEFAULT -1, develop_embedded_id INTEGER DEFAULT -1, comment TEXT);

CREATE INDEX PhotoEventIDIndex ON PhotoTable (event_id);
CREATE INDEX PhotoTableMD5FormatV2 on PhotoTable(md5, file_format);
CREATE INDEX PhotoTableThumbnailMD5Format on PhotoTable(thumbnail_md5, file_format);
CREATE INDEX PhotoTableThumbnailMD5MD5 on PhotoTable(thumbnail_md5, md5);
