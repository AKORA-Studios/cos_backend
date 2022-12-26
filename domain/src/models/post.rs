// domain/src/models.rs
use diesel::prelude::*;

use super::user::User;
use crate::schema::posts;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

// https://docs.diesel.rs/diesel/associations/index.html#traits
// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Serialize, Associations, PartialEq, Eq, Debug)]
#[diesel(belongs_to(User))]
// #[belongs_to(User, foreign_key = "photographer_id")]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub description: String,
    pub user_id: i32,
    pub downloads: i32,
    pub likes: i32,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    //https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
    pub created_at: SystemTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub description: String,
    pub user_id: i32,
    pub tags: Vec<String>,
    pub photographer_id: Option<i32>,
}
