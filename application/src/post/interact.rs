// application/src/post/comment.rs

use diesel::prelude::*;
use domain::models::{PostDownloads, PostLikes};

use rocket::response::status::NotFound;
use shared::response_models::ErrorMessageResponse;

/// IMPORTANT: User ID is required so users cannot delete arbitrary posts,
/// the user ID should be the ID of the user interacting with this API
pub fn delete_post(
    db_conn: &mut PgConnection,
    user_id: i32,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    use domain::schema::posts;

    let filter = posts::id.eq(post_id).and(posts::user_id.eq(user_id));

    let result = diesel::delete(posts::table).filter(filter).execute(db_conn);

    map_error(result)
}

pub fn like_post(
    db_conn: &mut PgConnection,
    user_id: i32,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    use domain::schema::post_likes;

    let val = PostLikes { post_id, user_id };

    let result = diesel::insert_into(post_likes::table)
        .values(&val)
        .execute(db_conn);

    map_error(result)
}

pub fn dislike_post(
    db_conn: &mut PgConnection,
    user_id: i32,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    use domain::schema::post_likes;

    let filter = post_likes::user_id
        .eq(user_id)
        .and(post_likes::post_id.eq(post_id));

    let result = diesel::delete(post_likes::table)
        .filter(filter)
        .execute(db_conn);

    map_error(result)
}

pub fn download_post(
    db_conn: &mut PgConnection,
    user_id: i32,
    post_id: i32,
) -> Result<(), NotFound<String>> {
    use domain::schema::post_downloads;

    let val = PostDownloads { post_id, user_id };

    let result = diesel::insert_into(post_downloads::table)
        .values(&val)
        .execute(db_conn);

    map_error(result)
}

/*








*/

fn map_error(result: diesel::QueryResult<usize>) -> Result<(), NotFound<String>> {
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
