// application/src/post/read.rs

use std::time::{Duration, SystemTime};

use diesel::prelude::*;
use domain::{models::Post, schema::posts::all_columns};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::MessageResponse;

pub fn view_post(post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts;

    match posts::table
        .find(post_id)
        .first::<Post>(&mut establish_connection())
    {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = MessageResponse {
                    message: format!("Error selecting post with id {} - {}", post_id, err),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_today_posts(count: usize) -> Vec<Post> {
    use domain::schema::posts::dsl::*;

    let result = posts
        .select(all_columns)
        .filter(created_at.gt(SystemTime::now() - Duration::from_secs(60 * 60 * 24)))
        .limit(count as i64)
        .load::<Post>(&mut establish_connection());

    match result {
        Ok(post_list) => post_list,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_user_posts(user_id: i32, count: usize) -> Vec<Post> {
    use domain::schema::posts;

    let result = posts::table
        .select(all_columns)
        .filter(posts::user_id.eq(user_id))
        .limit(count as i64)
        .load::<Post>(&mut establish_connection());

    match result {
        Ok(post_list) => post_list,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
