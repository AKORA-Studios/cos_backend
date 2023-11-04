// application/src/post/comment.rs

use crate::{map_sqlx_result, TaskResult};
use sqlx::PgPool;

/// IMPORTANT: User ID is required so users cannot delete arbitrary posts,
/// the user ID should be the ID of the user interacting with this API
pub async fn delete_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    map_sqlx_result(
        sqlx::query(
            r#"
        DELETE FROM posts
        WHERE id = $1 AND user_id = $2
        "#,
        )
        .bind(post_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map(|_| ()),
    )
}

pub async fn like_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    map_sqlx_result(
        sqlx::query(
            r#"
        INSERT INTO post_likes
        (post_id, user_id)
        VALUES ($1, $2)
        "#,
        )
        .bind(post_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map(|_| ()),
    )
}

pub async fn dislike_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    map_sqlx_result(
        sqlx::query(
            r#"
            DELETE FROM post_likes
            WHERE post_id = $1 AND user_id = $2
            "#,
        )
        .bind(post_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map(|_| ()),
    )
}

pub async fn download_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    map_sqlx_result(
        sqlx::query(
            r#"
            INSERT INTO post_downloads
            (post_id, user_id)
            VALUES ($1, $2)
            "#,
        )
        .bind(post_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map(|_| ()),
    )
}
