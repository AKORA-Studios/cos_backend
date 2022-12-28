// shared/src/response_models.rs

use domain::models::{DisplayUser, Event, Post};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostsResponse {
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostResponse {
    pub post: Post,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    pub user: DisplayUser,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EventRespone {
    pub event: Event,
}
