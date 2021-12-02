use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};
use crate::schema::notes;

#[derive(Debug, Queryable, Serialize)]
pub struct Note {
    pub id: i32,
    pub creator: String,
    pub note: String,
    pub ts: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "notes"]
pub struct NewNote {
    pub creator: String,
    pub note: String
}