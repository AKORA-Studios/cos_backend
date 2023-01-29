// api/src/post_handler.rs

use application::event::{create, read};
use domain::models::NewEvent;
use infrastructure::DbConn;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::EventRespone;

#[post("/events/new", format = "application/json", data = "<event>")]
pub async fn create_event_handler(conn: DbConn, event: Json<NewEvent>) -> Created<String> {
    conn.run(move |c| create::create_event(c, event)).await
}

#[get("/events/<event_id>")]
pub async fn view_event_handler(conn: DbConn, event_id: i32) -> Result<String, NotFound<String>> {
    let event = conn.run(move |c| read::view_event(c, event_id)).await?;
    let response = EventRespone { event };

    Ok(serde_json::to_string(&response).unwrap())
}
