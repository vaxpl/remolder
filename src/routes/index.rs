use crate::db;
use chrono::{Local, TimeZone};
use rocket::Route;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct FileExt {
    file: db::models::File,
    created_at_str: String,
}

#[derive(Debug, Serialize)]
struct HeaderCtx {
    show_upload: bool,
}

#[derive(Debug, Serialize)]
struct IndexCtx {
    header: HeaderCtx,
    list: Vec<FileExt>,
    size: usize,
}

#[get("/")]
pub(crate) fn index() -> Template {
    match db::files::all_files(&db::establish_connection()) {
        Ok(v) => {
            let new_list: Vec<FileExt> = v
                .into_iter()
                .map(|file| {
                    let created_at_local = Local.from_utc_datetime(&file.created_at);
                    let created_at_str = format!("{}", created_at_local.format("%F %T"));
                    FileExt {
                        file,
                        created_at_str,
                    }
                })
                .collect();

            let size = new_list.len();
            let header_ctx = HeaderCtx { show_upload: true };
            let index_ctx = IndexCtx {
                header: header_ctx,
                list: new_list,
                size,
            };
            Template::render("index", index_ctx)
        }
        Err(_) => Template::render("index", String::new()),
    }
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
