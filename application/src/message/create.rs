// application/src/message/create.rs

use diesel::prelude::*;
use domain::models::{Message, NewMessage};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::MessageResponse;

pub fn create_message(msg: Json<NewMessage>) -> Created<String> {
    use domain::schema::messages::dsl::*;

    let msg = msg.into_inner();

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
