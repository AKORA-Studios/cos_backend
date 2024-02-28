use chrono::{DateTime, Local};

use serde::Serialize;

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

    pub author_username: String,
    pub author_nickname: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct RawFullPost {
    pub id: i32,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub user_id: i32,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: DateTime<Local>,

    pub author_username: String,
    pub author_nickname: String,

    pub is_liked: bool,
    pub download_count: i64,
    pub like_count: i64,
    pub people_count: i64,
}

pub const POST_WITH_USER_COLUMNS: &'static str = r#"
    posts.id,
    posts.caption,
    posts.description,
    posts.user_id,
    posts.tags,
    posts.photographer_id,
    posts.lat,
    posts.lon,
    posts.created_at,
    
    users.username as author_username,
    users.nickname as author_nickname
"#;

impl RawFullPost {
    pub fn convert(&self) -> FullPost {
        FullPost {
            id: self.id,
            author: PostUserInfo {
                user_id: self.user_id,
                username: self.author_username.clone(),
                nickname: self.author_nickname.clone(),
            },
            caption: self.caption.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
            photographer_id: self.photographer_id,
            lat: self.lat,
            lon: self.lon,
            created_at: self.created_at,

            is_liked: self.is_liked,
            stats: PostStats {
                downloads: self.download_count,
                likes: self.like_count,
                people: self.people_count,
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FullPost {
    pub id: i32,
    pub author: PostUserInfo,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<Option<String>>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub created_at: DateTime<Local>,

    pub is_liked: bool,
    pub stats: PostStats,
    //pub depicted_people: Vec<i32>,
}

#[derive(Serialize, Debug)]
pub struct PostUserInfo {
    pub user_id: i32,
    pub username: String,
    pub nickname: String,
}

#[derive(Serialize, Debug)]
pub struct PostStats {
    pub downloads: i64,
    pub likes: i64,
    pub people: i64,
}
