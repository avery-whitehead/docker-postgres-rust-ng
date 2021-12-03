#![feature(decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use anyhow::Result;
use dotenv::dotenv;
use rocket::http::Method;
use rocket::routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;

pub mod db_pool;
pub mod schema;
pub mod models;
pub mod api;

fn main() -> Result<()> {
    dotenv()?;
    let database_url = env::var("DATABASE_URL")?;
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    rocket::ignite()
        .manage(db_pool::init(&database_url))
        .mount("/api/notes", routes![api::get_all, api::create, api::delete])
        .mount("/api/notes", rocket_cors::catch_all_options_routes())
        .attach(cors.clone())
        .manage(cors)
        .launch();
    Ok(())
}