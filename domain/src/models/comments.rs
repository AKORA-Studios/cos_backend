// domain/src/models.rs
use diesel::prelude::*;

use super::post::Post;
use crate::schema::comments;
use diesel::sql_types::Range;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Post)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub post_id: i32,
    pub created_at: SystemTime,
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
