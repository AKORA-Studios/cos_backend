// application/src/message/read.rs

use diesel::prelude::*;
use domain::models::Message;

use rocket::response::status::NotFound;

use crate::util::map_diesel_result;

pub fn view_message(
    db_conn: &mut PgConnection,
    message_id: i32,
) -> Result<Message, NotFound<String>> {
    use domain::schema::messages::dsl::*;

    let result = messages.find(message_id).first::<Message>(db_conn);

    map_diesel_result(result)
}

pub fn list_messages(
    db_conn: &mut PgConnection,
    user1_id: i32,
    user2_id: i32,
    limit: u32,
) -> Result<Vec<Message>, NotFound<String>> {
    use domain::schema::messages;

    let filter1 = messages::from_id
        .eq(user1_id)
        .and(messages::to_id.eq(user2_id));

    let filter2 = messages::from_id
        .eq(user2_id)
        .and(messages::to_id.eq(user1_id));

    let result = messages::table
        .filter(filter1.or(filter2))
        .limit(limit.into())
        .load::<Message>(db_conn);

    map_diesel_result(result)
}
