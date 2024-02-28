use chrono::{DateTime, Local};

use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct JoinedPostWithUser {
    pub id: i32,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub user_id: i32,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: DateTime<Local>,

    pub username: String,
    pub nickname: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FullJoinedPostWithCounts {
    pub id: i32,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub user_id: i32,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: DateTime<Local>,

    pub username: String,
    pub nickname: String,

    pub download_count: i64,
    pub like_count: i64,
    pub people_count: i64,
}

impl FullJoinedPostWithCounts {
    pub fn convert(&self, downloads: i64, likes: i64, depicted_people: i64) -> FullPost {
        FullPost {
            id: self.id,
            user: PostUserInfo {
                user_id: self.user_id,
                username: self.username.clone(),
                nickname: self.nickname.clone(),
            },
            caption: self.caption.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
            photographer_id: self.photographer_id,
            lat: self.lat,
            lon: self.lon,
            created_at: self.created_at,

            download_count: downloads,
            like_count: likes,
            people_count: depicted_people,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FullPost {
    pub id: i32,
    pub user: PostUserInfo,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: DateTime<Local>,

    pub download_count: i64,
    pub like_count: i64,
    pub people_count: i64,
    //pub depicted_people: Vec<i32>,
}

#[derive(Serialize, Debug)]
pub struct PostUserInfo {
    pub user_id: i32,
    pub username: String,
    pub nickname: String,
}