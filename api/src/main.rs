#![feature(decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub mod db_pool;
pub mod schema;
pub mod models;
pub mod api;

fn main() -> Result<()> {
    dotenv()?;
    let database_url = env::var("DATABASE_URL")?;
    rocket::ignite()
        .manage(db_pool::init(&database_url))
        .mount("/api", routes![api::get_all])
        .attach(api::CORS)
        .launch();
    Ok(())
}