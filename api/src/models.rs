use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct Note {
    pub id: i32,
    pub creator: String,
    pub note: String,
    pub ts: NaiveDateTime
}