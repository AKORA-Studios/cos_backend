// api/src/post_handler.rs

use application::auth::JWTClaims;
use application::user::{interact, login, modify, read, register};
use application::{OpResult, OpSuc};
use axum::extract::{Path, State};
use axum::Json;
use domain::models::{DisplayUser, PatchedUser};
use shared::request_models::{LoginCredentials, RegisterUser};
use shared::response_models::UserResponse;
use sqlx::postgres::PgPool;

/// POST /register - data = <user>
pub async fn register_user_handler(
    State(pool): State<PgPool>,
    Json(user): Json<RegisterUser>,
) -> OpResult<UserResponse, String> {
    register::register_user(&pool, user).await
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

#[get("/users/me")]
pub async fn view_me_handler(conn: DbConn, user: JWTClaims) -> Result<String, NotFound<String>> {
    let user = conn.run(move |c| read::view_user(c, user.user_id)).await?;
    let response = UserResponse { user };

    Ok(serde_json::to_string(&response).unwrap())
}

/*
#[patch("/users/me", format = "application/json", data = "<patch_data>")]
pub async fn patch_me_handler(
    conn: DbConn,
    user: JWTClaims,
    patch_data: Json<PatchedUser>,
) -> Result<String, NotFound<String>> {
    let user = conn
        .run(move |c| modify::modify_user(c, user.user_id, patch_data))
        .await?;
    let response = UserResponse { user };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/users/<user_id>/follow")]
pub async fn follow_user_handler(
    conn: DbConn,
    user: JWTClaims,
    user_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::follow_user(c, user.user_id, user_id))
        .await
}

#[put("/users/<user_id>/unfollow")]
pub async fn unfollow_user_handler(
    conn: DbConn,
    user: JWTClaims,
    user_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::unfollow_user(c, user.user_id, user_id))
        .await
}

#[put("/users/<user_id>/block")]
pub async fn block_user_handler(
    conn: DbConn,
    user: JWTClaims,
    user_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::block_user(c, user.user_id, user_id))
        .await
}

#[put("/users/<user_id>/unblock")]
pub async fn unblock_user_handler(
    conn: DbConn,
    user: JWTClaims,
    user_id: i32,
) -> Result<(), NotFound<String>> {
    conn.run(move |c| interact::unblock_user(c, user.user_id, user_id))
        .await
}
*/
