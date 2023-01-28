// api/src/post_handler.rs

use application::auth::JWTClaims;
use application::post::{comment, create, interact, read};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use shared::request_models::{NewComment, NewPost};
use shared::response_models::{CommentsRespone, FullPostResponse, FullPostsResponse};

#[post("/posts/new", format = "application/json", data = "<post>")]
pub fn create_post_handler(user: JWTClaims, post: Json<NewPost>) -> Created<String> {
    create::create_post(user.user_id, post)
}

#[get("/posts/<post_id>")]
pub fn view_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::view_post(post_id)?;
    let response = FullPostResponse { post };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/posts/<post_id>")]
pub fn delete_post_handler(user: JWTClaims, post_id: i32) -> Result<(), NotFound<String>> {
    interact::delete_post(user.user_id, post_id)
}

#[put("/posts/<post_id>/like")]
pub fn like_post_handler(user: JWTClaims, post_id: i32) -> Result<(), NotFound<String>> {
    interact::like_post(user.user_id, post_id)
}

#[put("/posts/<post_id>/dislike")]
pub fn dislike_post_handler(user: JWTClaims, post_id: i32) -> Result<(), NotFound<String>> {
    interact::dislike_post(user.user_id, post_id)
}

#[put("/posts/<post_id>/download")]
pub fn download_post_handler(user: JWTClaims, post_id: i32) -> Result<(), NotFound<String>> {
    interact::download_post(user.user_id, post_id)
}

// !TODO use post_id in url
#[post(
    "/posts/<post_id>/comments/new",
    format = "application/json",
    data = "<comment>"
)]
pub fn create_comment_handler(
    user: JWTClaims,
    post_id: i32,
    comment: Json<NewComment>,
) -> Created<String> {
    comment::create_post_comment(user.user_id, post_id, comment)
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

    let response = FullPostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/posts/recent?<limit>")]
pub fn list_recent_posts_handler(limit: Option<usize>) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_recent_posts(limit);

    let response = FullPostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/users/<user_id>/posts?<limit>")]
pub fn list_user_posts_handler(
    user_id: i32,
    limit: Option<usize>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_user_posts(user_id, limit);
    let response = FullPostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}
