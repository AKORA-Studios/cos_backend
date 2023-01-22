// application/src/message/create.rs

use diesel::prelude::*;
use domain::models::{Message, NewMessage};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::request_models::SendMessage;
use shared::response_models::MessageResponse;

use crate::auth::JWTClaims;

pub fn create_message(
    from_user: JWTClaims,
    to_user_id: i32,
    msg: Json<SendMessage>,
) -> Created<String> {
    use domain::schema::messages::dsl::*;

    let msg = msg.into_inner();
    let msg = NewMessage {
        attachment_id: msg.attachment_id,
        content: msg.content,
        from_id: from_user.user_id,
        reply_to: msg.reply_to,
        to_id: to_user_id,
    };

    match diesel::insert_into(messages)
        .values(&msg)
        .get_result::<Message>(&mut establish_connection())
    {
        Ok(message) => {
            let response = MessageResponse { message };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
