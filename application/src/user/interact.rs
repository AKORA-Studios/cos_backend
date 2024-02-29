// application/src/user/read.rs

use crate::TaskResult;
use sqlx::PgPool;

pub async fn follow_user(pool: &PgPool, user_id: i32, following_id: i32) -> TaskResult<(), String> {
    sqlx::query(
        r#"
        INSERT INTO user_follows (user_id, following_id)
        VALUES ($1, $2)
        ON CONFLICT(user_id, following_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(following_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn unfollow_user(
    pool: &PgPool,
    user_id: i32,
    following_id: i32,
) -> TaskResult<(), String> {
    sqlx::query(
        r#"
            DELETE FROM user_follows
            WHERE user_id = $1 AND following_id = $2
            "#,
    )
    .bind(user_id)
    .bind(following_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn block_user(pool: &PgPool, user_id: i32, blocked_id: i32) -> TaskResult<(), String> {
    sqlx::query(
        r#"
            INSERT INTO user_blocked (user_id, blocked_id)
            VALUES ($1, $2)
            ON CONFLICT(user_id, blocked_id) DO NOTHING
            "#,
    )
    .bind(user_id)
    .bind(blocked_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn unblock_user(pool: &PgPool, user_id: i32, blocked_id: i32) -> TaskResult<(), String> {
    sqlx::query(
        r#"
                DELETE FROM user_blocked
                WHERE user_id = $1 AND blocked_id = $2
                "#,
    )
    .bind(user_id)
    .bind(blocked_id)
    .execute(pool)
    .await?;

    Ok(())
}
