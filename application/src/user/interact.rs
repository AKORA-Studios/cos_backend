// application/src/user/read.rs

use crate::{map_sqlx_result, TaskResult};
use sqlx::PgPool;

pub async fn follow_user(pool: &PgPool, user_id: i32, following_id: i32) -> TaskResult<(), String> {
    let query = sqlx::query(
        r#"
        INSERT INTO user_follows (user_id, following_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(user_id)
    .bind(following_id);

    let result = query.execute(pool).await;

    map_sqlx_result(result.map(|_| ()))
}

pub async fn unfollow_user(
    pool: &PgPool,
    user_id: i32,
    following_id: i32,
) -> TaskResult<(), String> {
    let query = sqlx::query(
        r#"
            DELETE FROM user_follows
            WHERE user_id = $1 AND following_id = $2
            "#,
    )
    .bind(user_id)
    .bind(following_id);

    let result = query.execute(pool).await;

    map_sqlx_result(result.map(|_| ()))
}

pub async fn block_user(pool: &PgPool, user_id: i32, blocked_id: i32) -> TaskResult<(), String> {
    let query = sqlx::query(
        r#"
            INSERT INTO user_blocked (user_id, blocked_id)
            VALUES ($1, $2)
            "#,
    )
    .bind(user_id)
    .bind(blocked_id);

    let result = query.execute(pool).await;

    map_sqlx_result(result.map(|_| ()))
}

pub async fn unblock_user(pool: &PgPool, user_id: i32, blocked_id: i32) -> TaskResult<(), String> {
    let query = sqlx::query(
        r#"
                DELETE FROM user_blocked
                WHERE user_id = $1 AND blocked_id = $2
                "#,
    )
    .bind(user_id)
    .bind(blocked_id);

    let result = query.execute(pool).await;

    map_sqlx_result(result.map(|_| ()))
}
