// application/src/message/read.rs

use diesel::prelude::*;
use domain::models::Message;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;

use crate::util::map_diesel_result;

pub fn view_message(message_id: i32) -> Result<Message, NotFound<String>> {
    use domain::schema::messages::dsl::*;

    let result = messages
        .find(message_id)
        .first::<Message>(&mut establish_connection());

    map_diesel_result(result)
}
