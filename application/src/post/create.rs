// application/src/post/create.rs

use sqlx::PgPool;

use crate::{map_sqlx_result, TaskResult};
use domain::models::Post;

use shared::request_models::NewPost;
use shared::response_models::PostResponse;

pub async fn create_post(
    pool: &PgPool,
    user_id: i32,
    post: NewPost,
) -> TaskResult<PostResponse<Post>, String> {
    let result = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts
        (user_id, caption, description, tags, photographer_id, lat, lon)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
    "#,
    )
    .bind(user_id)
    .bind(post.caption)
    .bind(post.description)
    .bind(post.tags)
    .bind(post.photographer_id)
    .bind(post.lat)
    .bind(post.lon)
    .fetch_one(pool)
    .await;

    map_sqlx_result(result.map(|p| PostResponse { post: p }))
}
