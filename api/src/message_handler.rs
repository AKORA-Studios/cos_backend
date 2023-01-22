// api/src/post_handler.rs

use application::auth::JWTClaims;
use application::message::{create, read};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::request_models::SendMessage;
use shared::response_models::MessagesResponse;

#[post(
    "/users/<to_user_id>/messages/new",
    format = "application/json",
    data = "<msg>"
)]
pub fn create_message_handler(
    user: JWTClaims,
    to_user_id: i32,
    msg: Json<SendMessage>,
) -> Created<String> {
    create::create_message(user, to_user_id, msg)
}

#[get("/users/<user_id>/messages?<limit>")]
pub fn list_conversation_handler(
    req_user: JWTClaims,
    user_id: i32,
    limit: Option<u32>,
) -> Result<String, NotFound<String>> {
    let limit = limit.unwrap_or(20).clamp(1, 100);

    let messages = read::list_messages(req_user.user_id, user_id, limit)?;
    let response = MessagesResponse { messages };

    Ok(serde_json::to_string(&response).unwrap())
}
