// use maplit::hashset;
use rocket::config::Value;
// use rocket::request::{FromRequest, Outcome, Request};
use std::collections::BTreeMap;

const DEFAULT_DATA_DIR: &str = "data";
const DEFAULT_FILE_DIR: &str = "data/files";
const DEFAULT_TEMP_DIR: &str = "temp";

#[derive(Clone, Debug, Default)]
pub struct Storage {
    pub data_dir: String,
    pub file_dir: String,
    pub temp_dir: String,
}

impl From<&BTreeMap<String, Value>> for Storage {
    fn from(table: &BTreeMap<String, Value>) -> Self {
        Self {
            data_dir: table_get_string(table, "data_dir", DEFAULT_DATA_DIR),
            file_dir: table_get_string(table, "file_dir", DEFAULT_FILE_DIR),
            temp_dir: table_get_string(table, "temp_dir", DEFAULT_TEMP_DIR),
        }
    }
}

/// Returns the string specified by name in the table.
fn table_get_string(table: &BTreeMap<String, Value>, name: &str, def_val: &str) -> String {
    table
        .get(name)
        .map_or(def_val, |x| x.as_str().unwrap_or(def_val))
        .to_string()
}
