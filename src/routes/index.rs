use crate::db;
use rocket::Route;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HeaderCtx {
    show_upload: bool,
}

#[derive(Debug, Serialize)]
struct IndexCtx {
    header: HeaderCtx,
    list: Vec<db::models::File>,
    size: usize,
}

#[get("/")]
pub(crate) fn index() -> Template {
    match db::files::all_files(&db::establish_connection()) {
        Ok(v) => {
            let size = v.len();
            let header_ctx = HeaderCtx { show_upload: true };
            let index_ctx = IndexCtx {
                header: header_ctx,
                list: v,
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
