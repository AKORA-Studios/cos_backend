// application/src/post/read.rs

use domain::models::{FullJoinedPostWithCounts, FullPost};
use sqlx::PgPool;

use crate::{map_sqlx_result, TaskResult};

const POST_WITH_USER_COLUMNS_AND_COUNTS: &'static str = r#"
    posts.id,
    posts.caption,
    posts.description,
    posts.user_id,
    posts.tags,
    posts.photographer_id,
    posts.lat,
    posts.lon,
    posts.created_at,
    users.username,
    users.nickname,
    (SELECT COUNT(*) FROM post_downloads WHERE post_id = posts.id)
        AS download_count,
    (SELECT COUNT(*) FROM post_likes WHERE post_id = posts.id)
        AS like_count,
    (SELECT COUNT(*) FROM post_depicted_people WHERE post_id = posts.id)
        AS people_count
"#;

pub async fn prepare_post_statements(pool: &PgPool) -> Result<(), sqlx::Error> {
    let prepare_view_post = format!(
        r#"
        PREPARE view_post(int) AS
            SELECT {POST_WITH_USER_COLUMNS_AND_COUNTS}
            FROM posts INNER JOIN users ON posts.user_id = users.id
            WHERE posts.id = $1;
    "#
    );

    let prepare_list_recent_posts = format!(
        r#"
        PREPARE list_recent_posts(int) AS
            SELECT {POST_WITH_USER_COLUMNS_AND_COUNTS}
            FROM posts INNER JOIN users ON posts.user_id = users.id
            ORDER BY posts.created_at DESC
            LIMIT $1;
    "#
    );

    let prepare_list_recent_posts_by_user = format!(
        r#"
        PREPARE list_recent_posts_by_user(int, int) AS
            SELECT {POST_WITH_USER_COLUMNS_AND_COUNTS}
            FROM posts INNER JOIN users ON posts.user_id = users.id
            WHERE posts.user_id = $1
            ORDER BY posts.created_at DESC
            LIMIT $2;
    "#
    );

    use futures::try_join;

    let _ = try_join!(
        sqlx::query(&prepare_view_post).execute(pool),
        sqlx::query(&prepare_list_recent_posts).execute(pool),
        sqlx::query(&prepare_list_recent_posts_by_user).execute(pool),
    )?;

    Ok(())
}

pub async fn view_post(pool: &PgPool, post_id: i32) -> TaskResult<FullPost, String> {
    let result = sqlx::query_as::<_, FullJoinedPostWithCounts>("EXECUTE view_post($1);")
        .bind(post_id)
        .fetch_one(pool)
        .await;

    map_sqlx_result(result.map(|p| p.convert(p.download_count, p.like_count, p.people_count)))
}

pub async fn list_recent_posts(
    pool: &PgPool,
    limit: i32,
) -> TaskResult<Vec<FullJoinedPostWithCounts>, String> {
    let result = sqlx::query_as::<_, FullJoinedPostWithCounts>("EXECUTE list_recent_posts($1);")
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
) -> TaskResult<Vec<FullJoinedPostWithCounts>, String> {
    let result =
        sqlx::query_as::<_, FullJoinedPostWithCounts>("EXECUTE list_recent_posts_by_user($1, $2);")
            .bind(user_id)
            .bind(limit)
            .fetch_all(pool)
            .await;

    map_sqlx_result(result)
}
