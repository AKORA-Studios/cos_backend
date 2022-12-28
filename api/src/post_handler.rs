// api/src/post_handler.rs

use application::post::{comment, create, read};
use domain::models::{NewComment, NewPost};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::{CommentsRespone, PostResponse, PostsResponse};

#[get("/posts/<post_id>")]
pub fn view_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::view_post(post_id)?;
    let response = PostResponse { post };

    Ok(serde_json::to_string(&response).unwrap())
}

// !TODO use post_id in url
#[post("/posts/comments/new", format = "application/json", data = "<comment>")]
pub fn create_comment_handler(comment: Json<NewComment>) -> Created<String> {
    comment::create_post_comment(comment)
}

#[get("/posts/<post_id>/comments/recent?<limit>")]
pub fn list_recent_comments_handler(
    post_id: i32,
    limit: Option<usize>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let comments = comment::list_recent_comments(post_id, limit);

    let response = CommentsRespone { comments };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/posts/today?<limit>")]
pub fn list_today_posts_handler(limit: Option<usize>) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_today_posts(limit);

    let response = PostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/posts/recent?<limit>")]
pub fn list_recent_posts_handler(limit: Option<usize>) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_recent_posts(limit);

    let response = PostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/users/<user_id>/posts?<limit>")]
pub fn list_user_posts_handler(
    user_id: i32,
    limit: Option<usize>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_user_posts(user_id, limit);
    let response = PostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/posts/new", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<NewPost>) -> Created<String> {
    create::create_post(post)
}
