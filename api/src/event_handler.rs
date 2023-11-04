// api/src/post_handler.rs

use application::{
    event::{create, read},
    OpResult, OpSuc,
};
use axum::{
    extract::{Path, State},
    Json,
};
use domain::models::NewEvent;
use shared::response_models::EventResponse;
use sqlx::PgPool;

/// POST /events/new        <event>
pub async fn create_event_handler(
    State(pool): State<PgPool>,
    Json(event): Json<NewEvent>,
) -> OpResult<EventResponse, String> {
    create::create_event(&pool, event)
        .await
        .map(|e| OpSuc::Created(e))
}

/// get /events/<event_id>
pub async fn view_event_handler(
    State(pool): State<PgPool>,
    Path(event_id): Path<i32>,
) -> OpResult<EventResponse, String> {
    read::view_event(&pool, event_id)
        .await
        .map(|e| OpSuc::Read(e))
}
