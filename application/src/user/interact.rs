// application/src/user/read.rs

use diesel::prelude::*;
use domain::models::{UserBlocked, UserFollows};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::ErrorMessageResponse;

pub fn follow_user(user_id: i32, following_id: i32) -> Result<(), NotFound<String>> {
    use domain::schema::user_follows;

    let val = UserFollows {
        user_id,
        following_id,
    };

    let result = diesel::insert_into(user_follows::table)
        .values(&val)
        .execute(&mut establish_connection());

    map_error(result)
}

pub fn unfollow_user(user_id: i32, following_id: i32) -> Result<(), NotFound<String>> {
    use domain::schema::user_follows;

    let filter = user_follows::user_id
        .eq(user_id)
        .and(user_follows::following_id.eq(following_id));

    let result = diesel::delete(user_follows::table)
        .filter(filter)
        .execute(&mut establish_connection());

    map_error(result)
}

pub fn block_user(user_id: i32, blocked_id: i32) -> Result<(), NotFound<String>> {
    use domain::schema::user_blocked;

    let val = UserBlocked {
        user_id,
        blocked_id,
    };

    let result = diesel::insert_into(user_blocked::table)
        .values(&val)
        .execute(&mut establish_connection());

    map_error(result)
}

pub fn unblock_user(user_id: i32, blocked_id: i32) -> Result<(), NotFound<String>> {
    use domain::schema::user_blocked;

    let filter = user_blocked::user_id
        .eq(user_id)
        .and(user_blocked::blocked_id.eq(blocked_id));

    let result = diesel::delete(user_blocked::table)
        .filter(filter)
        .execute(&mut establish_connection());

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
                        message: format!("User not found."),
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
