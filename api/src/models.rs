use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Note {
    pub id: i32,
    pub creator: String,
    pub note: String,
    pub ts: NaiveDateTime
}