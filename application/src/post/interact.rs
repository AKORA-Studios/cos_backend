// application/src/post/comment.rs

use diesel::prelude::*;
use domain::models::{PostDownloads, PostLikes};

use rocket::response::status::NotFound;
use shared::response_models::ErrorMessageResponse;

/// IMPORTANT: User ID is required so users cannot delete arbitrary posts,
/// the user ID should be the ID of the user interacting with this API
pub async fn delete_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    let filter = posts::id.eq(post_id).and(posts::user_id.eq(user_id));

    let result = diesel::delete(posts::table).filter(filter).execute(db_conn);

    map_sqlx_result(result)
}

pub async fn like_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    let val = PostLikes { post_id, user_id };

    let result = diesel::insert_into(post_likes::table)
        .values(&val)
        .execute(db_conn);

    map_sqlx_result(result)
}

pub async fn dislike_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    let filter = post_likes::user_id
        .eq(user_id)
        .and(post_likes::post_id.eq(post_id));

    let result = diesel::delete(post_likes::table)
        .filter(filter)
        .execute(db_conn);

    map_sqlx_result(result)
}

pub async fn download_post(pool: &PgPool, user_id: i32, post_id: i32) -> TaskResult<(), String> {
    let val = PostDownloads { post_id, user_id };

    let result = diesel::insert_into(post_downloads::table)
        .values(&val)
        .execute(db_conn);

    map_sqlx_result(result)
}

/*








*/

fn map_error(result: diesel::QueryResult<usize>) -> TaskResult<(), String> {
    match result {
        Ok(_rows) => Ok(()),
        Err(err) => match &err {
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                // Post already liked
                diesel::result::DatabaseErrorKind::UniqueViolation => Ok(()),
                // Post ID incorrect
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                    let response = ErrorMessageResponse {
                        message: format!("Post not found."),
                    };
                    Err(NotFound(serde_json::to_string(&response).unwrap()))
                }
                _ => panic!("Database error - {}", err),
            },
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
