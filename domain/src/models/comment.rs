use serde::Serialize;
// domain/src/models.rs

use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(sqlx::FromRow, Serialize, PartialEq, Eq, Debug)]
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
pub struct InsertableComment {
    pub content: String,
    pub user_id: i32,
    pub post_id: i32,
    pub reply_to: Option<i32>,
}
