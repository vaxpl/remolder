use crate::db;
use crate::models::{ApiFailure, ApiResult, ApiSuccess, Mime, Storage};
use crate::utils::{FileDigest, TeeWriter};
use rocket::http::{ContentType, MediaType};
use rocket::{Data, Route, State};
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::fs::File;
use std::path::{Path, PathBuf};

const DEFAULT_FILE_SIZE_MAX: u64 = 1_073_741_824;

#[get("/up")]
pub(crate) fn upload_index() -> Template {
    Template::render("upload", String::new())
}

#[post("/up", data = "<data>")]
pub(crate) fn upload_by_post(
    content_type: &ContentType,
    data: Data,
    storage: State<Storage>,
) -> ApiResult<db::models::File, String, ()> {
    let options = MultipartFormDataOptions {
        temporary_dir: PathBuf::from(&storage.temp_dir),
        allowed_fields: vec![
            MultipartFormDataField::file("file").size_limit(DEFAULT_FILE_SIZE_MAX),
            MultipartFormDataField::text("remote_path"),
        ],
    };

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)?;

    let files = multipart_form_data
        .files
        .get("file")
        .ok_or("The `file` field does not exists!")?;

    let file = files.iter().next().ok_or("You did not send any files!")?;

    let origin_file_name = file
        .file_name
        .as_ref()
        .ok_or("The file name could not be empty!")
        .map(|s| s.to_string())?;

    let mime_str = file
        .content_type
        .as_ref()
        .map_or("application/octet-stream".to_string(), |x| format!("{}", x));

    // Get the file content size
    let size = file.path.metadata().map(|x| x.len())?;

    // Calculate the MD5 digest of the file
    let hash = file.path.md5_str();

    // Replace the filename with `remote_path` if specified
    let path_str = if let Some(v) = multipart_form_data.texts.get("remote_path") {
        let text = &v[0].text;
        if text.is_empty() {
            origin_file_name
        } else {
            Clone::clone(text)
        }
    } else {
        origin_file_name
    };

    // Make dest file path with hash
    let dest_file = Path::new(&storage.file_dir).join(&hash);

    // Check if the dest file exists
    if dest_file.exists() {
        // Remove the temporary file and return failure
        // std::fs::remove_file(&temp_file)?;
        let errors = format!("File {} already exists", hash);
        return Err(errors.into());
    }

    // Move the file from temporary to dest
    std::fs::rename(&file.path, &dest_file)?;

    // Add file information to database
    let file = db::files::add_file(
        &db::establish_connection(),
        &hash,
        &path_str,
        &mime_str,
        size as i32,
    )?;

    // Returns the success message with file record
    Ok(ApiSuccess::new(Some(file), None))
}

#[put("/up/<path..>", data = "<data>")]
pub(crate) fn upload_by_put(
    path: PathBuf,
    data: Data,
    mime: Option<Mime>,
    storage: State<Storage>,
) -> ApiResult<db::models::File, String, ()> {
    let path_str = format!("{}", path.display());
    // Build the mime type string
    let mime_from_path = path
        .extension()
        .map(|v| MediaType::from_extension(v.to_str().unwrap()).map(Mime))
        .flatten();
    let mime = mime
        .or(mime_from_path)
        .or(Some(Mime(MediaType::Any)))
        .unwrap();
    let mime_str = format!("{}", mime);

    // The temporary file path
    let temp_file = Path::new(&storage.temp_dir).join(&path);

    // Create the temporary file to save stream data
    let mut file = File::create(&temp_file)?;

    // Create MD5 hasher to digest the file content
    let mut hash = md5::Context::new();
    let size = {
        let mut tee = TeeWriter::new(&mut file, &mut hash);
        data.stream_to(&mut tee)?
    };

    // Make dest file path with hash
    let hash = format!("{:x}", hash.compute());
    let dest_file = Path::new(&storage.file_dir).join(&hash);

    // Check if the dest file exists
    if dest_file.exists() {
        // Remove the temporary file and return failure
        std::fs::remove_file(&temp_file)?;
        let errors = format!("File {} already exists", hash);
        return Err(ApiFailure::new(Some(errors)));
    }

    // Move the file from temporary to dest
    std::fs::rename(&temp_file, &dest_file)?;

    // Add file information to database
    let file = db::files::add_file(
        &db::establish_connection(),
        &hash,
        &path_str,
        &mime_str,
        size as i32,
    )?;

    // Returns the success message with file record
    Ok(ApiSuccess::new(Some(file), None))
}

pub fn routes() -> Vec<Route> {
    routes![upload_index, upload_by_post, upload_by_put]
}
