// shared/src/response_models.rs

use domain::models::{Comment, DisplayUser, Event, FullPost, Message, Post};
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
pub struct PostResponse {
    pub post: Post,
}

#[derive(Serialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
pub struct FullPostResponse {
    pub post: FullPost,
}

#[derive(Serialize)]
pub struct FullPostsResponse {
    pub posts: Vec<FullPost>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user: DisplayUser,
}

#[derive(Serialize)]
pub struct EventRespone {
    pub event: Event,
}

#[derive(Serialize)]
pub struct CommentRespone {
    pub comment: Comment,
}

#[derive(Serialize)]
pub struct CommentsRespone {
    pub comments: Vec<Comment>,
}

#[derive(Serialize)]
pub struct TokenRespone {
    pub token: String,
}
