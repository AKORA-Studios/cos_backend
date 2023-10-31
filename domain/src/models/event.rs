// domain/src/models.rs
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub lat: f64,
    pub lon: f64,
}
