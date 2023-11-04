// application/src/post/comment.rs

use domain::models::Comment;

use sqlx::PgPool;

use crate::{map_sqlx_result, TaskResult};

use shared::request_models::NewComment;
use shared::response_models::CommentResponse;

pub async fn create_post_comment(
    pool: &PgPool,
    user_id: i32,
    post_id: i32,
    comment: NewComment,
) -> TaskResult<CommentResponse, String> {
    let result = sqlx::query_as::<_, Comment>(
        r#"
        INSERT INTO comments
        (content, user_id, post_id, reply_to)
        VALUES ($1, $2, $3, $4)
        RETURNING *
    "#,
    )
    .bind(comment.content)
    .bind(user_id)
    .bind(post_id)
    .bind(comment.reply_to)
    .fetch_one(pool)
    .await;

    map_sqlx_result(result.map(|c| CommentResponse { comment: c }))
}

pub async fn list_recent_comments(
    pool: &PgPool,
    c_post_id: i32,
    limit: i32,
) -> TaskResult<Vec<Comment>, String> {
    let result = sqlx::query_as::<_, Comment>(
        r#"
        SELECT * FROM comments
        WHERE post_id = $1
        ORDER BY created_at DESC
        LIMIT $2
    "#,
    )
    .bind(c_post_id)
    .bind(limit.abs())
    .fetch_all(pool)
    .await;

    map_sqlx_result(result)
}
