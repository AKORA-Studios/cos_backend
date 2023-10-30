use std::time::SystemTime;

use serde::Serialize;

#[derive(sqlx::FromRow, Debug)]
pub struct JoinedPostWithUser {
    pub id: i32,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub user_id: i32,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: SystemTime,

    pub username: String,
    pub nickname: String,
}

impl JoinedPostWithUser {
    pub fn convert(&self, downloads: i64, likes: i64, depicted_people: Vec<i32>) -> FullPost {
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

            downloads,
            likes,
            depicted_people,
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
    pub created_at: SystemTime,

    pub downloads: i64,
    pub likes: i64,
    pub depicted_people: Vec<i32>,
}

#[derive(Serialize, Debug)]
pub struct PostUserInfo {
    pub user_id: i32,
    pub username: String,
    pub nickname: String,
}

/*
type PostWithUserColumns = (
    posts::id,
    posts::caption,
    posts::description,
    posts::user_id,
    posts::tags,
    posts::photographer_id,
    posts::lat,
    posts::lon,
    posts::created_at,
    users::username,
    users::nickname,
);

pub const POST_WITH_USER_COLUMNS: PostWithUserColumns = (
    posts::id,
    posts::caption,
    posts::description,
    posts::user_id,
    posts::tags,
    posts::photographer_id,
    posts::lat,
    posts::lon,
    posts::created_at,
    users::username,
    users::nickname,
);
*/
