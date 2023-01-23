// domain/src/models.rs
use diesel::prelude::*;

use super::post::Post;
use crate::schema::comments;
use rocket::serde::Serialize;
use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Serialize, Associations, PartialEq, Eq, Debug)]
#[diesel(belongs_to(Post))]
#[diesel(table_name = comments)]
#[diesel(primary_key(id))]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub post_id: i32,

    pub reply_to: Option<i32>,
    pub upvotes: i32,
    pub created_at: SystemTime,
}

// !TODO replies

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct InsertableComment {
    pub content: String,
    pub user_id: i32,
    pub post_id: i32,
    pub reply_to: Option<i32>,
}
