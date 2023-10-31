// application/src/event/create.rs

use domain::models::{Event, NewEvent};

use crate::{OpErr, OpResult};
use shared::response_models::EventRespone;
use sqlx::{self, PgPool};

pub async fn create_event(db_conn: &PgPool, event: NewEvent) -> OpResult<String, _> {
    match sqlx::query_as::<Event>(
        r#"
        INSERT INTO "events" (name, start_time, end_time, lat, lon)
        values ($1, $2, $3, $4, $5)
    "#,
    )
    .bind(event.name)
    .bind(event.start_time)
    .bind(event.end_time)
    .bind(event.lat)
    .bind(event.lon)
    .execute(db_conn)
    .await
    {
        Ok(event) => {
            let response = EventRespone { event };
            Ok(serde_json::to_string(&response).unwrap())
        }
        Err(err) => OpErr::InternalError(err),
    }
}
