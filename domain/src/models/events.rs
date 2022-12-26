// domain/src/models.rs
use diesel::prelude::*;

use crate::schema::events;
use diesel::sql_types::Range;
use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Debug)]
#[diesel(table_name = events)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub during: Range<SystemTime>,
    pub lat: f64,
    pub lon: f64,
}

/*
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub genre: String,
}
*/
