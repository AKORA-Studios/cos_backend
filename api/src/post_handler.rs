// api/src/post_handler.rs

use application::post::*;
use application::{OpResult, OpSuc};
use axum::extract::{Path, State};
use axum::Json;
use domain::models::{FullJoinedPostWithCounts, FullPost, Post};
use shared::response_models::{CommentResponse, PostResponse, PostsResponse};
use sqlx::postgres::PgPool;

use shared::request_models::{NewComment, NewPost};
use shared::response_models::CommentsResponse;

use crate::extractors::auth::Claims;
use crate::extractors::limit::Limit;

/// POST /posts/new   <post>
pub async fn create_post_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Json(post): Json<NewPost>,
) -> OpResult<PostResponse<Post>, String> {
    create::create_post(&pool, claims.user_id, post)
        .await
        .map(|p| OpSuc::Created(p))
}

/// get /posts/<post_id>
pub async fn view_post_handler(
    State(pool): State<PgPool>,
    Path(post_id): Path<i32>,
) -> OpResult<PostResponse<FullPost>, String> {
    let post = read::view_post(&pool, post_id).await?;
    let response = PostResponse { post };

    Ok(OpSuc::Read(response))
}

/// delete /posts/<post_id>
pub async fn delete_post_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Path(post_id): Path<i32>,
) -> OpResult<(), String> {
    interact::delete_post(&pool, claims.user_id, post_id)
        .await
        .map(|_| OpSuc::Deleted(()))
}

/// put /posts/<post_id>/like
pub async fn like_post_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Path(post_id): Path<i32>,
) -> OpResult<(), String> {
    interact::like_post(&pool, claims.user_id, post_id)
        .await
        .map(|_| OpSuc::Created(()))
}

/// put /posts/<post_id>/dislike
pub async fn dislike_post_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Path(post_id): Path<i32>,
) -> OpResult<(), String> {
    interact::dislike_post(&pool, claims.user_id, post_id)
        .await
        .map(|_| OpSuc::Deleted(()))
}

/// put /posts/<post_id>/download
pub async fn download_post_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Path(post_id): Path<i32>,
) -> OpResult<(), String> {
    interact::download_post(&pool, claims.user_id, post_id)
        .await
        .map(|_| OpSuc::Created(()))
}

// !TODO use post_id in url
/// POST /posts/<post_id>/comments/new     <comment>
pub async fn create_comment_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Path(post_id): Path<i32>,
    Json(comment): Json<NewComment>,
) -> OpResult<CommentResponse, String> {
    comment::create_post_comment(&pool, claims.user_id, post_id, comment)
        .await
        .map(|c| OpSuc::Created(c))
}

/// get /posts/<post_id>/comments/recent?<limit>
pub async fn list_recent_comments_handler(
    State(pool): State<PgPool>,
    Path(post_id): Path<i32>,
    Limit(limit): Limit,
) -> OpResult<CommentsResponse, String> {
    let limit = limit.unwrap_or(25);
    let comments = comment::list_recent_comments(&pool, post_id, limit).await?;

    let response = CommentsResponse { comments };

    Ok(OpSuc::Read(response))
}

/*
/// get /posts/today?<limit>
pub async fn list_today_posts_handler(
    State(pool): State<PgPool>,
    Query(limit): Query<Option<i64>>,
) -> OpResult<FullPostsResponse, String> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_today_posts(&pool, limit).await;

    let response = FullPostsResponse { posts };

    Ok(OpSuc::Read(response))
}
*/

/// get /posts/recent?<limit>
pub async fn list_recent_posts_handler(
    State(pool): State<PgPool>,
    Limit(limit): Limit,
) -> OpResult<PostsResponse<FullJoinedPostWithCounts>, String> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_recent_posts(&pool, limit).await?;

    let response = PostsResponse { posts };

    Ok(OpSuc::Read(response))
}

/// get /users/<user_id>/posts?<limit>
pub async fn list_user_posts_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    Limit(limit): Limit,
) -> OpResult<PostsResponse<FullJoinedPostWithCounts>, String> {
    let limit = limit.unwrap_or(25);
    let posts = read::list_user_posts(&pool, user_id, limit).await?;
    let response = PostsResponse { posts };

    Ok(OpSuc::Read(response))
}
