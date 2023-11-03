// api/src/post_handler.rs

use application::user::{interact, modify, read, register};
use application::{OpResult, OpSuc};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use domain::models::PatchedUser;
use shared::request_models::RegisterUser;
use shared::response_models::UserResponse;
use sqlx::postgres::PgPool;

use crate::auth::Claims;

pub async fn status() -> StatusCode {
    StatusCode::OK
}

/// POST /register - data = <user>
pub async fn register_user_handler(
    State(pool): State<PgPool>,
    Json(user): Json<RegisterUser>,
) -> OpResult<UserResponse, String> {
    register::register_user(&pool, user)
        .await
        .map(|u| OpSuc::Created(u))
}

/// GET /users/:user_id
pub async fn view_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> OpResult<UserResponse, String> {
    let user = read::view_user(&pool, user_id).await?;

    let response = UserResponse { user };

    Ok(OpSuc::Read(response))
}

/// GET /users/me
pub async fn view_me_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
) -> OpResult<UserResponse, String> {
    let user = read::view_user(&pool, claims.user_id).await?;
    let response = UserResponse { user };

    Ok(OpSuc::Read(response))
}

/// PATH /users/me <patch_data>
pub async fn patch_me_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Json(patch_data): Json<PatchedUser>,
) -> OpResult<UserResponse, String> {
    let user = modify::modify_user(&pool, claims.user_id, patch_data).await?;
    let response = UserResponse { user };

    Ok(OpSuc::Updated(response))
}

/// PUT /users/:user_id/follow
pub async fn follow_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    Claims(claims): Claims,
) -> OpResult<(), String> {
    interact::follow_user(&pool, claims.user_id, user_id)
        .await
        .map(|_| OpSuc::Created(()))
}

/// PUT /users/:user_id/unfollow
pub async fn unfollow_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    Claims(claims): Claims,
) -> OpResult<(), String> {
    interact::unfollow_user(&pool, claims.user_id, user_id)
        .await
        .map(|_| OpSuc::Deleted(()))
}

/// PUT /users/:user_id/block
pub async fn block_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    Claims(claims): Claims,
) -> OpResult<(), String> {
    interact::block_user(&pool, claims.user_id, user_id)
        .await
        .map(|_| OpSuc::Created(()))
}

/// PUT /users/:user_id/unblock
pub async fn unblock_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    Claims(claims): Claims,
) -> OpResult<(), String> {
    interact::unblock_user(&pool, claims.user_id, user_id)
        .await
        .map(|_| OpSuc::Deleted(()))
}
