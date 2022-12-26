// domain/src/models.rs
use diesel::prelude::*;

use super::user::User;
use crate::schema::messages;
use rocket::serde::Deserialize;
use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Associations, PartialEq, Eq, Debug)]
#[diesel(belongs_to(User, foreign_key = to_id))]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub from_id: i32,
    pub to_id: i32,
    pub created_at: SystemTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub content: String,
    pub from_id: i32,
    pub to_id: i32,
}
