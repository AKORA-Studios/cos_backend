// application/src/message/create.rs

use domain::models::Message;

use shared::request_models::NewMessage;
use shared::response_models::MessageResponse;
use sqlx::PgPool;

use crate::TaskResult;

pub async fn create_message(
    pool: &PgPool,
    from_user_id: i32,
    to_user_id: i32,
    msg: NewMessage,
) -> TaskResult<MessageResponse, String> {
    let message = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages
        (content, attachment_id, reply_to, from_id, to_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
    "#,
    )
    .bind(msg.content)
    .bind(msg.attachment_id)
    .bind(msg.reply_to)
    .bind(from_user_id)
    .bind(to_user_id)
    .fetch_one(pool)
    .await?;

    Ok(MessageResponse { message })
}
