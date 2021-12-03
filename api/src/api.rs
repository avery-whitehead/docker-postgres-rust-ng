use diesel;
use diesel::prelude::*;
use crate::db_pool::DbConn;
use crate::models::{Note, NewNote};
use crate::schema::notes;
use crate::schema::notes::dsl::*;
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::ops::Deref;

#[get("/")]
pub fn get_all(conn: DbConn) -> QueryResult<Json<Vec<Note>>> {
    notes.load(&*conn)
        .map(|ns| Json(ns))
}

#[post("/", data = "<new_note>")]
pub fn create(conn: DbConn, new_note: Json<NewNote>) -> QueryResult<Json<Note>> {
    diesel::insert_into(notes::table)
        .values(&*new_note)
        .get_result::<Note>(&*conn)
        .map(|n| Json(n))
}

#[delete("/<note_id>")]
pub fn delete(conn: DbConn, note_id: i32) -> Result<status::NoContent, Status> {
    diesel::delete(notes::table.find(note_id))
        .execute(&*conn)
        .map(|_| status::NoContent)
        .map_err(|_| Status::new(500, "Error"))
}

impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}