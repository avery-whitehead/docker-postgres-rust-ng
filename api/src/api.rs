use diesel;
use diesel::prelude::*;
use crate::db_pool::DbConn;
use crate::models::Note;
use crate::schema::notes::dsl::*;
use rocket::get;
use rocket_contrib::json::Json;
use std::ops::Deref;

#[get("/")]
pub fn get_all(conn: DbConn) -> QueryResult<Json<Vec<Note>>> {
    notes.load(&*conn)
        .map(|ns| Json(ns))
}

impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}