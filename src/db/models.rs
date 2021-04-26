use super::schema::files;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct File {
    pub id: i32,
    pub hash: String,
    pub path: String,
    pub size: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub hash: &'a str,
    pub path: &'a str,
    pub size: i32,
}
