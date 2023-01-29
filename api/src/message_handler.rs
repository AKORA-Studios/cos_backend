// api/src/post_handler.rs

use application::auth::JWTClaims;
use application::message::{create, read};
use infrastructure::DbConn;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::request_models::NewMessage;
use shared::response_models::MessagesResponse;

#[post(
    "/users/<to_user_id>/messages/new",
    format = "application/json",
    data = "<msg>"
)]
pub async fn create_message_handler(
    conn: DbConn,
    user: JWTClaims,
    to_user_id: i32,
    msg: Json<NewMessage>,
) -> Created<String> {
    conn.run(move |c| create::create_message(c, user, to_user_id, msg))
        .await
}

#[get("/users/<user_id>/messages?<limit>")]
pub async fn list_conversation_handler(
    conn: DbConn,
    req_user: JWTClaims,
    user_id: i32,
    limit: Option<u32>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(20).clamp(1, 100);

    let messages = conn
        .run(move |c| read::list_messages(c, req_user.user_id, user_id, limit))
        .await?;
    let response = MessagesResponse { messages };

    Ok(serde_json::to_string(&response).unwrap())
}
