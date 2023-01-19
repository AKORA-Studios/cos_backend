// domain/src/models.rs

use crate::schema::users;
use diesel::prelude::*;

use serde::Serialize;
use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Serialize, PartialEq, Eq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub password_hash: String,
    pub email: String,
    //https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
    pub created_at: SystemTime,

    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
    pub myanimelist_username: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub nickname: String,
    pub password_hash: String,
    pub email: String,

    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
    pub myanimelist_username: Option<String>,
}

#[derive(Identifiable, Queryable, Serialize, PartialEq, Eq, Debug)]
#[diesel(table_name = users)]
pub struct DisplayUser {
    pub id: i32,
    pub username: String,
    pub nickname: String,

    //https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
    pub created_at: SystemTime,
    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
    pub myanimelist_username: Option<String>,
}

type DisplayUserColumns = (
    users::id,
    users::username,
    users::nickname,
    users::created_at,
    users::twitter_username,
    users::instagram_username,
    users::tiktok_username,
    users::onlyfans_username,
    users::snapchat_username,
    users::youtube_username,
    users::myanimelist_username,
);

pub const DISPLAY_USER_COLUMNS: DisplayUserColumns = (
    users::id,
    users::username,
    users::nickname,
    users::created_at,
    users::twitter_username,
    users::instagram_username,
    users::tiktok_username,
    users::onlyfans_username,
    users::snapchat_username,
    users::youtube_username,
    users::myanimelist_username,
);
