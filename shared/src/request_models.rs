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
