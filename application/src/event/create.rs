// application/src/event/create.rs

use diesel::prelude::*;
use domain::models::{Event, NewEvent};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::EventRespone;

pub fn create_event(event: Json<NewEvent>) -> Created<String> {
    use domain::schema::events::dsl::*;

    let event = event.into_inner();

    match diesel::insert_into(events)
        .values(&event)
        .get_result::<Event>(&mut establish_connection())
    {
        Ok(event) => {
            let response = EventRespone { event };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
