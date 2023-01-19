// shared/src/response_models.rs

use domain::models::{Comment, DisplayUser, Event, FullPost, Message, Post};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorMessageResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageResponse {
    pub message: Message,
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostResponse {
    pub post: Post,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostsResponse {
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FullPostResponse {
    pub post: FullPost,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FullPostsResponse {
    pub posts: Vec<FullPost>,
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommentRespone {
    pub comment: Comment,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommentsRespone {
    pub comments: Vec<Comment>,
}
