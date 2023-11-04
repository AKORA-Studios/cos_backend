// application/src/message/read.rs

use domain::models::Message;

use sqlx::PgPool;

use crate::{map_sqlx_result, TaskResult};

pub async fn view_message(pool: &PgPool, message_id: i32) -> TaskResult<Message, String> {
    let result = sqlx::query_as::<_, Message>(r#"SELECT * FROM messages WHERE id = $1"#)
        .bind(message_id)
        .fetch_one(pool)
        .await;

    map_sqlx_result(result)
}

pub async fn list_messages(
    pool: &PgPool,
    user1_id: i32,
    user2_id: i32,
    limit: i32,
) -> TaskResult<Vec<Message>, String> {
    let result = sqlx::query_as::<_, Message>(
        r#"
    SELECT * FROM messages
    WHERE (from_id = $1 AND to_id = $2) OR (from_id = $2 AND to_id = $1)
    ORDER BY created_at DESC
    LIMIT $3
    "#,
    )
    .bind(user1_id)
    .bind(user2_id)
    .bind(limit)
    .fetch_all(pool)
    .await;

    map_sqlx_result(result)
}
