// application/src/post/read.rs

use domain::models::{FullPost, JoinedPostWithUser, RawFullPost, POST_WITH_USER_COLUMNS};
use sqlx::PgPool;

use crate::{map_sqlx_result, TaskResult};

// !TODO Also return users if they already liked a post or not

pub async fn view_post(
    pool: &PgPool,
    post_id: i32,
    viewer_id: Option<i32>,
) -> TaskResult<FullPost, String> {
    let sql = format!(
        r#"
        SELECT {POST_WITH_USER_COLUMNS},
        (SELECT EXISTS(SELECT FROM post_likes WHERE post_id = $1 AND user_id = $2 )) AS is_liked,
        (SELECT COUNT(*) FROM post_downloads WHERE post_id = $1) AS download_count,
        (SELECT COUNT(*) FROM post_likes WHERE post_id = $1) AS like_count,
        (SELECT COUNT(*) FROM post_depicted_people WHERE post_id = $1) AS people_count

        FROM posts INNER JOIN users ON posts.user_id = users.id

        WHERE posts.id = $1
        "#,
    );

    let result = sqlx::query_as::<_, RawFullPost>(&sql)
        .bind(post_id)
        .bind(viewer_id.unwrap_or(0)) // TODO: Probably not the smartest assumption
        .fetch_one(pool)
        .await;

    map_sqlx_result(result.map(|p| p.convert()))
}

pub async fn list_recent_posts(
    pool: &PgPool,
    limit: i32,
) -> TaskResult<Vec<JoinedPostWithUser>, String> {
    let sql = format!(
        r#"SELECT {POST_WITH_USER_COLUMNS}
            FROM posts INNER JOIN users ON posts.user_id = users.id
            ORDER BY posts.created_at DESC
            LIMIT $1
            "#
    );

    let result = sqlx::query_as::<_, JoinedPostWithUser>(&sql)
        .bind(limit)
        .fetch_all(pool)
        .await;

    map_sqlx_result(result)
}

/*
pub async fn list_today_posts(
    pool: &PgPool,
    limit: usize,
) -> TaskResult<Vec<JoinedPostWithUser>, String> {
    use domain::schema::posts::dsl::*;

    let result = posts
        .select(all_columns)
        .filter(created_at.gt(SystemTime::now() - Duration::from_secs(60 * 60 * 24)))
        .limit(limit as i64)
        .inner_join(users::table.on(users::id.eq(user_id)))
        .select(POST_WITH_USER_COLUMNS)
        .load::<JoinedPostWithUser>(db_conn);

    let sql = format!(
        r#"SELECT {POST_WITH_USER_COLUMNS}
                FROM posts INNER JOIN users ON posts.user_id = users.id
                ORDER BY posts.created_at DESC
                LIMIT $1
                "#
    );

    let result = sqlx::query_as::<_, JoinedPostWithUser>(&sql)
        .bind(limit)
        .fetch_all(pool)
        .await;

    map_sqlx_result(result)
}
 */

pub async fn list_user_posts(
    pool: &PgPool,
    user_id: i32,
    limit: i32,
) -> TaskResult<Vec<JoinedPostWithUser>, String> {
    let sql = format!(
        r#"SELECT {POST_WITH_USER_COLUMNS}
                FROM posts INNER JOIN users ON posts.user_id = users.id
                WHERE posts.user_id = $1
                ORDER BY posts.created_at DESC
                LIMIT $2
                "#
    );

    let result = sqlx::query_as::<_, JoinedPostWithUser>(&sql)
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await;

    map_sqlx_result(result)
}

/*

fn get_post_info(
    post: &JoinedPostWithUser,
    conn: &mut PgConnection,
) -> diesel::result::QueryResult<(i64, i64, Vec<i32>)> {
    let downloads: i64 = post_downloads::table
        .filter(post_downloads::post_id.eq(post.id))
        .count()
        .get_result(conn)?;

    "SELECT COUNT(*) FROM post_downloads WHERE post_id = $1";

    let likes: i64 = post_likes::table
        .filter(post_likes::post_id.eq(post.id))
        .count()
        .get_result(conn)?;

    let depicted_people = post_depicted_people::table
        .filter(post_depicted_people::post_id.eq(post.id))
        .select(post_depicted_people::user_id)
        .load::<i32>(conn)?;

    Ok((downloads, likes, depicted_people))
}
*/
