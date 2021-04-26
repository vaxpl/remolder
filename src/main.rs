#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use remolder::models::Storage;
use remolder::routes;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Storage Config", |rocket| {
            let storage = Storage::from(rocket.config().get_table("storage").unwrap());
            Ok(rocket.manage(storage))
        }))
        .mount("/", routes::index::routes())
        .mount("/assets", StaticFiles::from("assets"))
}

fn main() {
    rocket().launch();
}
