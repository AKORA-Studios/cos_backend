// api/src/post_handler.rs

use application::auth::JWTClaims;
use application::post::{comment, create, interact, read};
use infrastructure::DbConn;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use shared::request_models::{NewComment, NewPost};
use shared::response_models::{CommentsRespone, FullPostResponse, FullPostsResponse};

#[post("/posts/new", format = "application/json", data = "<post>")]
pub async fn create_post_handler(
    conn: DbConn,
    user: JWTClaims,
    post: Json<NewPost>,
) -> Created<String> {
    conn.run(move |c| create::create_post(c, user.user_id, post))
        .await
}

#[get("/posts/<post_id>")]
pub async fn view_post_handler(conn: DbConn, post_id: i32) -> Result<String, NotFound<String>> {
    let post = conn.run(move |c| read::view_post(c, post_id)).await?;
    let response = FullPostResponse { post };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/posts/<post_id>")]
pub async fn delete_post_handler(
    conn: DbConn,
    user: JWTClaims,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::delete_post(c, user.user_id, post_id))
        .await
}

#[put("/posts/<post_id>/like")]
pub async fn like_post_handler(
    conn: DbConn,
    user: JWTClaims,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::like_post(c, user.user_id, post_id))
        .await
}

#[put("/posts/<post_id>/dislike")]
pub async fn dislike_post_handler(
    conn: DbConn,
    user: JWTClaims,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::dislike_post(c, user.user_id, post_id))
        .await
}

#[put("/posts/<post_id>/download")]
pub async fn download_post_handler(
    conn: DbConn,
    user: JWTClaims,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::download_post(c, user.user_id, post_id))
        .await
}

// !TODO use post_id in url
#[post(
    "/posts/<post_id>/comments/new",
    format = "application/json",
    data = "<comment>"
)]
pub async fn create_comment_handler(
    conn: DbConn,
    user: JWTClaims,
    post_id: i32,
    comment: Json<NewComment>,
) -> Created<String> {
    conn.run(move |c| comment::create_post_comment(c, user.user_id, post_id, comment))
        .await
}

#[get("/posts/<post_id>/comments/recent?<limit>")]
pub async fn list_recent_comments_handler(
    conn: DbConn,
    post_id: i32,
    limit: Option<usize>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let comments = conn
        .run(move |c| comment::list_recent_comments(c, post_id, limit))
        .await;

    let response = CommentsRespone { comments };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/posts/today?<limit>")]
pub async fn list_today_posts_handler(
    conn: DbConn,
    limit: Option<usize>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = conn.run(move |c| read::list_today_posts(c, limit)).await;

    let response = FullPostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/posts/recent?<limit>")]
pub async fn list_recent_posts_handler(
    conn: DbConn,
    limit: Option<usize>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = conn.run(move |c| read::list_recent_posts(c, limit)).await;

    let response = FullPostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/users/<user_id>/posts?<limit>")]
pub async fn list_user_posts_handler(
    conn: DbConn,
    user_id: i32,
    limit: Option<usize>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(25);
    let posts = conn
        .run(move |c| read::list_user_posts(c, user_id, limit))
        .await;
    let response = FullPostsResponse { posts };

    Ok(serde_json::to_string(&response).unwrap())
}
