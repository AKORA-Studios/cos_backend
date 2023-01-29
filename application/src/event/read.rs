// application/src/event/read.rs

use diesel::prelude::*;
use domain::models::Event;

use rocket::response::status::NotFound;

use crate::util::map_diesel_result;

pub fn view_event(db_conn: &mut PgConnection, event_id: i32) -> Result<Event, NotFound<String>> {
    use domain::schema::events::dsl::*;

    let result = events.find(event_id).first::<Event>(db_conn);

    map_diesel_result(result)
}
