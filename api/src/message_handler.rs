// api/src/post_handler.rs

use application::message::{create, read};
use domain::models::NewMessage;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::MessageResponse;

#[post("/messages/new", format = "application/json", data = "<user>")]
pub fn create_message_handler(user: Json<NewMessage>) -> Created<String> {
    create::create_message(user)
}

#[get("/messages/<message_id>")]
pub fn view_message_handler(message_id: i32) -> Result<String, NotFound<String>> {
    let message = read::view_message(message_id)?;
    let response = MessageResponse { message };

    Ok(serde_json::to_string(&response).unwrap())
}
