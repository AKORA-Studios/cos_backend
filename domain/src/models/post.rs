// domain/src/models.rs
use chrono::{DateTime, Local};
use serde::Serialize;

// https://docs.diesel.rs/diesel/associations/index.html#traits
// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Post {
    pub id: i32,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub user_id: i32,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    //https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
    pub created_at: DateTime<Local>,
}

pub struct InsertablePost {
    pub caption: Option<String>,
    pub description: Option<String>,
    pub user_id: i32,
    pub tags: Vec<String>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}
