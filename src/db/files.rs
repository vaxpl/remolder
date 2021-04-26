use super::models::{File, NewFile};
use diesel::prelude::*;

/// Add a new file to the database.
pub fn add_file(
    conn: &SqliteConnection,
    hash_val: &str,
    path_val: &str,
    size_val: i32,
) -> Result<File, diesel::result::Error> {
    use super::schema::files;
    use super::schema::files::dsl::*;

    let new_file = NewFile {
        hash: hash_val,
        path: path_val,
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
