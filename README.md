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

TODO: POSTs with EXIF

- SHA hashes

TODO: object stores.

TODO: REST Client

TODO: Directory search