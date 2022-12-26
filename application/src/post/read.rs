// application/src/post/read.rs

use diesel::prelude::*;
use domain::{models::Post, schema::posts::all_columns};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_post(post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts;

    match posts::table
        .find(post_id)
        .first::<Post>(&mut establish_connection())
    {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting post with id {} - {}",
                        post_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_posts(user_id: i32, count: usize) -> Vec<Post> {
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
