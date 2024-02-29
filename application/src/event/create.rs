// application/src/event/create.rs

use domain::models::{Event, NewEvent};

use crate::TaskResult;
use shared::response_models::EventResponse;
use sqlx::{self, PgPool};

pub async fn create_event(db_conn: &PgPool, event: NewEvent) -> TaskResult<EventResponse, String> {
    let event = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO "events" (name, start_time, end_time, lat, lon)
        VALUES ($1, $2, $3, $4, $5)
    "#,
    )
    .bind(event.name)
    .bind(event.start_time)
    .bind(event.end_time)
    .bind(event.lat)
    .bind(event.lon)
    .fetch_one(db_conn)
    .await?;

    Ok(EventResponse { event })
}
