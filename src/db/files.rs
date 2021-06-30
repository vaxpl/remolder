use super::models::{File, NewFile};
use diesel::prelude::*;

/// Add a new file to the database.
pub fn add_file(
    conn: &SqliteConnection,
    hash_val: &str,
    path_val: &str,
    mime_val: &str,
    size_val: i32,
) -> Result<File, diesel::result::Error> {
    use super::schema::files;
    use super::schema::files::dsl::*;

    let new_file = NewFile {
        hash: hash_val,
        path: path_val,
        mime: mime_val,
        size: size_val,
    };

    diesel::insert_into(files::table)
        .values(&new_file)
        .execute(conn)?;

    files::table
        .filter(hash.eq(hash_val))
        .limit(1)
        .load::<File>(conn)
        .map(|mut v| v.pop().unwrap())
}

/// Returns all files from the database.
pub fn all_files(conn: &SqliteConnection) -> Result<Vec<File>, diesel::result::Error> {
    use super::schema::files;
    use super::schema::files::id;

    files::table.order(id.desc()).load::<File>(conn)
}

/// Find and returns a file by hash or path from the database.
pub fn find_file(conn: &SqliteConnection, path_val: &str) -> Result<File, diesel::result::Error> {
    use super::schema::files;
    use super::schema::files::dsl::*;

    let mut v = files::table
        .filter(hash.eq(path_val).or(path.eq(path_val)))
        .order(id.desc())
        .limit(1)
        .load::<File>(conn)?;
    v.pop().ok_or(diesel::result::Error::NotFound)
}

/// Find and returns a file by id from the database.
pub fn find_file_by_id(
    conn: &SqliteConnection,
    id_val: i32,
) -> Result<File, diesel::result::Error> {
    use super::schema::files;
    use super::schema::files::dsl::*;

    let mut v = files::table
        .filter(id.eq(id_val))
        .limit(1)
        .load::<File>(conn)?;
    v.pop().ok_or(diesel::result::Error::NotFound)
}
