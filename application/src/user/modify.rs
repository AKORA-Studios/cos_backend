// application/src/user/read.rs

use domain::models::{DisplayUser, PatchedUser, DISPLAY_USER_COLUMNS};

use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{map_sqlx_result, OpErr, TaskResult};

pub async fn modify_user(
    conn: &PgPool,
    user_id: i32,
    patch_data: PatchedUser,
) -> TaskResult<DisplayUser, String> {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(r#"UPDATE "users" SET "#);

    let mut sep = query.separated(", ");

    if patch_data.nickname.is_some() {
        sep.push("nickname = ")
            .push_bind_unseparated(patch_data.nickname);
    };
    if patch_data.twitter_username.is_some() {
        sep.push("twitter_username = ")
            .push_bind_unseparated(patch_data.twitter_username);
    }
    if patch_data.instagram_username.is_some() {
        sep.push("instagram_username = ")
            .push_bind_unseparated(patch_data.instagram_username);
    }
    if patch_data.tiktok_username.is_some() {
        sep.push("tiktok_username = ")
            .push_bind_unseparated(patch_data.tiktok_username);
    }
    if patch_data.onlyfans_username.is_some() {
        sep.push("onlyfans_username = ")
            .push_bind_unseparated(patch_data.onlyfans_username);
    }
    if patch_data.snapchat_username.is_some() {
        sep.push("snapchat_username = ")
            .push_bind_unseparated(patch_data.snapchat_username);
    }
    if patch_data.youtube_username.is_some() {
        sep.push("youtube_username = ")
            .push_bind_unseparated(patch_data.youtube_username);
    }
    if patch_data.myanimelist_username.is_some() {
        sep.push("myanimelist_username = ")
            .push_bind_unseparated(patch_data.myanimelist_username);
    }

    query.push(" WHERE id = ").push_bind(user_id);
    query.push(format!(" RETURNING {}", DISPLAY_USER_COLUMNS));

    let finished_query = query.build_query_as::<DisplayUser>();

    let result = finished_query.fetch_one(conn).await;

    map_sqlx_result(result)
}
