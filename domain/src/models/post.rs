// domain/src/models.rs
use diesel::prelude::*;

use super::user::User;
use crate::schema::posts;
use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

// https://docs.diesel.rs/diesel/associations/index.html#traits
// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Serialize, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
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
    pub created_at: SystemTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub caption: Option<String>,
    pub description: Option<String>,
    pub user_id: i32,
    pub tags: Vec<String>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}
