#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod db;
pub mod models;
pub mod routes;
pub mod utils;
