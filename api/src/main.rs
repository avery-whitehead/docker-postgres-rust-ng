#[macro_use]
extern crate diesel;

use anyhow::{anyhow, Result};
use crate::schema::notes::dsl::*;
use crate::models::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

fn main() -> Result<()> {
    let connection = establish_connection()?;
    let results = notes.load::<Note>(&connection)?;
    println!("{:?}", results);
    Ok(())
}

pub fn establish_connection() -> Result<PgConnection> {
    dotenv()?;
    let database_url = env::var("DATABASE_URL")?;
    PgConnection::establish(&database_url).map_err(|e| anyhow!("Err {}", e))
}
