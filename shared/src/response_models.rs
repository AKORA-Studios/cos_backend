// shared/src/response_models.rs

use domain::models::{Comment, DisplayUser, Event, Message};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: Message,
}

#[derive(Serialize)]
pub struct MessagesResponse {
    pub messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct PostResponse<P: Serialize> {
    pub post: P,
}

#[derive(Serialize)]
pub struct PostsResponse<P: Serialize> {
    pub posts: Vec<P>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user: DisplayUser,
}

#[derive(Serialize)]
pub struct EventResponse {
    pub event: Event,
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub comment: Comment,
}

#[derive(Serialize)]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}
