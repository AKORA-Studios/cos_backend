// api/src/post_handler.rs

use application::event::{create, read};
use domain::models::NewEvent;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::EventRespone;

#[post("/events/new", format = "application/json", data = "<event>")]
pub fn create_event_handler(event: Json<NewEvent>) -> Created<String> {
    create::create_event(event)
}

#[get("/events/<event_id>")]
pub fn view_event_handler(event_id: i32) -> Result<String, NotFound<String>> {
    let event = read::view_event(event_id)?;
    let response = EventRespone { event };

    Ok(serde_json::to_string(&response).unwrap())
}
