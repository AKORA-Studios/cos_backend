// api/src/post_handler.rs

use application::message::{create, read};
use application::{OpResult, OpSuc};
use axum::extract::{Path, State};
use axum::Json;

use shared::request_models::NewMessage;
use shared::response_models::{MessageResponse, MessagesResponse};
use sqlx::PgPool;

use crate::extractors::auth::Claims;
use crate::extractors::limit::Limit;

/// POST /users/<to_user_id>/messages/new       <msg>
pub async fn create_message_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Path(user_id): Path<i32>,
    Json(msg): Json<NewMessage>,
) -> OpResult<MessageResponse, String> {
    create::create_message(&pool, claims.user_id, user_id, msg)
        .await
        .map(|m| OpSuc::Created(m))
}

/// get /users/<user_id>/messages?<limit>
pub async fn list_conversation_handler(
    State(pool): State<PgPool>,
    Claims(claims): Claims,
    Path(user_id): Path<i32>,
    Limit(limit): Limit,
) -> OpResult<MessagesResponse, String> {
    let limit = limit.unwrap_or(20);

    let messages = read::list_messages(&pool, claims.user_id, user_id, limit).await?;
    let response = MessagesResponse { messages };

    Ok(OpSuc::Read(response))
}
