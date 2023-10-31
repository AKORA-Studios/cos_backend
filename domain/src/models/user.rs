// domain/src/models.rs
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(sqlx::FromRow, Serialize, PartialEq, Eq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub password_hash: String,
    pub email: String,
    //https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
    pub created_at: DateTime<Local>,

    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
    pub myanimelist_username: Option<String>,
}

pub struct InsertableUser {
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

#[derive(sqlx::FromRow, Serialize, PartialEq, Eq, Debug)]
pub struct DisplayUser {
    pub id: i32,
    pub username: String,
    pub nickname: String,

    //https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
    pub created_at: DateTime<Local>,
    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
    pub myanimelist_username: Option<String>,
}
pub const DISPLAY_USER_COLUMNS: &'static str = r#"
    id,
    username,
    nickname,
    created_at,
    nickname,
    twitter_username,
    instagram_username,
    tiktok_username,
    onlyfans_username,
    snapchat_username,
    youtube_username,
    myanimelist_username
"#;

/*

pub type DisplayUserColumns = (
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

*/
#[derive(Deserialize)]
pub struct PatchedUser {
    pub nickname: Option<String>,

    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
    pub myanimelist_username: Option<String>,
}
