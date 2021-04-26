use crate::db;
use crate::models::{ApiMessage, Storage};
use crate::utils::TeeWriter;
use rocket::{Data, Route, State};
use rocket_contrib::json::Json;
use std::fs::File;
use std::path::{Path, PathBuf};

#[get("/")]
pub(crate) fn index() -> String {
    "Hello".to_string()
}

type SuccessResp = Json<ApiMessage<db::models::File, (), ()>>;
type FailureResp = Json<ApiMessage<(), String, ()>>;
type UploadResult = Result<SuccessResp, FailureResp>;

#[put("/upload/<path..>", data = "<data>")]
pub(crate) fn upload(path: PathBuf, data: Data, storage: State<Storage>) -> UploadResult {
    // Make temp file path
    let temp_file = Path::new(&storage.temp_dir).join(&path);
    let mut file =
        File::create(&temp_file).map_err(|e| Json(ApiMessage::failure(Some(e.to_string()))))?;
    // Create MD5 hasher to digest the file content
    let mut hash = md5::Context::new();
    let size = {
        let mut tee = TeeWriter::new(&mut file, &mut hash);
        data.stream_to(&mut tee)
            .map_err(|e| Json(ApiMessage::failure(Some(e.to_string()))))?
    };
    // Make dest file path with hash
    let hash = format!("{:x}", hash.compute());
    let dest_file = Path::new(&storage.file_dir).join(&hash);
    // Check if the dest file exists
    if dest_file.exists() {
        // Remove the temp file and return failure
        std::fs::remove_file(&temp_file)
            .map_err(|e| Json(ApiMessage::failure(Some(e.to_string()))))?;
        Err(Json(ApiMessage::failure(Some(format!(
            "File {} already exists",
            hash
        )))))
    } else {
        // Move the file from temp to dest
        std::fs::rename(&temp_file, &dest_file)
            .map_err(|e| Json(ApiMessage::failure(Some(e.to_string()))))?;
        // Add file information to database
        let file = db::files::add_file(
            &db::establish_connection(),
            &hash,
            path.to_str().unwrap(),
            size as i32,
        )
        .map_err(|e| Json(ApiMessage::failure(Some(e.to_string()))))?;
        // Returns the success message with file record
        Ok(Json(ApiMessage::success(Some(file), None)))
    }
}

pub fn routes() -> Vec<Route> {
    routes![index, upload]
}
