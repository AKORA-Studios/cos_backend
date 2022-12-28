// application/src/post/comment.rs

use diesel::prelude::*;
use domain::models::{Comment, NewComment};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::CommentRespone;

pub fn create_post_comment(comment: Json<NewComment>) -> Created<String> {
    use domain::schema::comments::dsl::*;

    let comment = comment.into_inner();

    match diesel::insert_into(comments)
        .values(&comment)
        .get_result::<Comment>(&mut establish_connection())
    {
        Ok(comment) => {
            let response = CommentRespone { comment };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_recent_comments(c_post_id: i32, limit: usize) -> Vec<Comment> {
    use domain::schema::comments::dsl::*;

    let result = comments
        .filter(post_id.eq(c_post_id))
        .order(created_at.desc())
        .limit(limit as i64)
        .load::<Comment>(&mut establish_connection());

    match result {
        Ok(comment_list) => comment_list,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
