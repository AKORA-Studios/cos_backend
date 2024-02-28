// application/src/post/read.rs

use domain::models::{FullPost, RawFullPost};
use sqlx::{Acquire, PgPool};

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
    users.username AS author_username,
    users.nickname AS author_nickname,
    (SELECT COUNT(*) FROM post_downloads WHERE post_id = posts.id)
        AS download_count,
    (SELECT COUNT(*) FROM post_likes WHERE post_id = posts.id)
        AS like_count,
    (SELECT COUNT(*) FROM post_depicted_people WHERE post_id = posts.id)
        AS people_count
"#;

use const_format::formatcp;

const SQL_VIEW_POST: &'static str = formatcp!(
    r#"
        SELECT {POST_WITH_USER_COLUMNS_AND_COUNTS}
        FROM posts INNER JOIN users ON posts.user_id = users.id
        WHERE posts.id = $1;
    "#
);

const SQL_LIST_RECENT_POSTS: &'static str = formatcp!(
    r#"
        SELECT {POST_WITH_USER_COLUMNS_AND_COUNTS}
        FROM posts INNER JOIN users ON posts.user_id = users.id
        ORDER BY posts.created_at DESC
        LIMIT $1;
    "#
);

const SQL_LIST_RECENT_POSTS_BY_USER: &'static str = formatcp!(
    r#"
        SELECT {POST_WITH_USER_COLUMNS_AND_COUNTS}
        FROM posts INNER JOIN users ON posts.user_id = users.id
        WHERE posts.user_id = $1
        ORDER BY posts.created_at DESC
        LIMIT $2;
    "#
);

pub async fn prepare_post_statements(conn: &mut sqlx::PgConnection) -> Result<(), sqlx::Error> {
    let prepare_view_post = format!("PREPARE view_post(int) AS {SQL_VIEW_POST}");

    let prepare_list_recent_posts =
        format!("PREPARE list_recent_posts(int) AS {SQL_LIST_RECENT_POSTS}");

    let prepare_list_recent_posts_by_user =
        format!("PREPARE list_recent_posts_by_user(int, int) AS {SQL_LIST_RECENT_POSTS_BY_USER}");

    let mut trans = conn.begin().await?;

    sqlx::query(&prepare_view_post).execute(&mut *trans).await?;
    sqlx::query(&prepare_list_recent_posts)
        .execute(&mut *trans)
        .await?;
    sqlx::query(&prepare_list_recent_posts_by_user)
        .execute(&mut *trans)
        .await?;

    trans.commit().await?;

    Ok(())
}

pub async fn view_post(pool: &PgPool, post_id: i32) -> TaskResult<FullPost, String> {
    let result = sqlx::query_as::<_, RawFullPost>(SQL_VIEW_POST)
        .bind(post_id)
        //     .bind(viewer_id.unwrap_or(0)) // TODO: Probably not the smartest assumption
        .fetch_one(pool)
        .await;

    map_sqlx_result(result.map(|p| p.convert()))
}

pub async fn list_recent_posts(pool: &PgPool, limit: i32) -> TaskResult<Vec<FullPost>, String> {
    let result = sqlx::query_as::<_, RawFullPost>(SQL_LIST_RECENT_POSTS)
        .bind(limit)
        .fetch_all(pool)
        .await;

    map_sqlx_result(result.map(|p| p.into_iter().map(|p| p.convert()).collect()))
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
) -> TaskResult<Vec<FullPost>, String> {
    let result = sqlx::query_as::<_, RawFullPost>(SQL_LIST_RECENT_POSTS_BY_USER)
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await;

    map_sqlx_result(result.map(|p| p.into_iter().map(|p| p.convert()).collect()))
}
