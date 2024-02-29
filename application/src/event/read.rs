// application/src/event/read.rs

use domain::models::Event;

use shared::response_models::EventResponse;
use sqlx::PgPool;

use crate::TaskResult;

pub async fn view_event(pool: &PgPool, event_id: i32) -> TaskResult<EventResponse, String> {
    let event = sqlx::query_as::<_, Event>(
        r#"
        SELECT * FROM events
        WHERE id = $1
    "#,
    )
    .bind(event_id)
    .fetch_one(pool)
    .await?;

    Ok(EventResponse { event })
}
