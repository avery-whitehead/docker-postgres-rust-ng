use diesel;
use diesel::prelude::*;
use crate::db_pool::DbConn;
use crate::models::Note;
use crate::schema::notes::dsl::*;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::get;
use rocket::http::Header;
use rocket_contrib::json::Json;
use std::ops::Deref;

pub struct CORS;

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

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}