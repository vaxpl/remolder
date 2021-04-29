remolder
========

Self-contained file storage web service written with Rust + Rocket.

Features
--------

* [x] All files listing.
* [x] Upload file with PUT method.
* [x] Upload file with POST method.
* [x] Download file with path, hash or id.

Development
-----------

Initialize the database and directories first.

```sh
# Setup directories
mkdir -p data data/db data/files data/temp
# Setup database
echo DATABASE_URL=data/db/remolder.db > .env
diesel migration run
```
