// domain/src/models.rs

use crate::schema::users;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub password_hash: String,
    //https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
    pub created_at: SystemTime,
    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
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