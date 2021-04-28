use crate::db;
use crate::models::Storage;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::{Request, Response, Route, State};
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct DownloadFile(PathBuf, ContentType);

impl<'r> Responder<'r> for DownloadFile {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let file = File::open(self.0).map_err(|_| Status::InternalServerError)?;
        Response::build().header(self.1).streamed_body(file).ok()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct FileNotFound(String, String);

impl<'r> Responder<'r> for FileNotFound {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Template::render("download-failure", &self)
            .respond_to(req)
            .map(|mut r| {
                r.set_status(Status::NotFound);
                r
            })
    }
}

#[get("/dl/<path..>", rank = 2)]
pub(crate) fn download(
    path: PathBuf,
    storage: State<Storage>,
) -> Result<DownloadFile, FileNotFound> {
    let path_str = format!("{}", path.display());
    db::files::find_file(&db::establish_connection(), &path_str).map_or_else(
        |e| Err(FileNotFound(path_str, e.to_string())),
        |v| {
            Ok(DownloadFile(
                Path::new(&storage.file_dir).join(&v.hash),
                ContentType::parse_flexible(&v.mime).unwrap_or(ContentType::Binary),
            ))
        },
    )
}

#[get("/dl/<id>", rank = 1)]
pub(crate) fn download_by_id(
    id: i32,
    storage: State<Storage>,
) -> Result<DownloadFile, FileNotFound> {
    db::files::find_file_by_id(&db::establish_connection(), id).map_or_else(
        |e| Err(FileNotFound(format!("# {}", id), e.to_string())),
        |v| {
            Ok(DownloadFile(
                Path::new(&storage.file_dir).join(&v.hash),
                ContentType::parse_flexible(&v.mime).unwrap_or(ContentType::Binary),
            ))
        },
    )
}

pub fn routes() -> Vec<Route> {
    routes![download, download_by_id]
}
