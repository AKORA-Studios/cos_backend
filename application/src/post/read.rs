// application/src/post/read.rs

use std::time::{Duration, SystemTime};

use diesel::prelude::*;
use domain::{models::Post, schema::posts::all_columns};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;

use crate::util::map_diesel_result;

pub fn view_post(post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    let result = posts
        .find(post_id)
        .first::<Post>(&mut establish_connection());

    map_diesel_result(result)
}

pub fn list_today_posts(limit: usize) -> Vec<Post> {
    use domain::schema::posts::dsl::*;

    let result = posts
        .select(all_columns)
        .filter(created_at.gt(SystemTime::now() - Duration::from_secs(60 * 60 * 24)))
        .limit(limit as i64)
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

pub fn list_user_posts(user_id: i32, limit: usize) -> Vec<Post> {
    use domain::schema::posts;

    let result = posts::table
        .select(all_columns)
        .filter(posts::user_id.eq(user_id))
        .limit(limit as i64)
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
