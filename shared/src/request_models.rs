use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterUser {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub email: String,

    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub onlyfans_username: Option<String>,
    pub snapchat_username: Option<String>,
    pub youtube_username: Option<String>,
    pub myanimelist_username: Option<String>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(untagged)]
pub enum LoginCredentials {
    UsernameCredentials { username: String, password: String },
    EmailCredentials { email: String, password: String },
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewMessage {
    pub content: String,
    pub attachment_id: Option<i32>,
    pub reply_to: Option<i32>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewComment {
    pub content: String,
    pub reply_to: Option<i32>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewPost {
    pub caption: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub photographer_id: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}
