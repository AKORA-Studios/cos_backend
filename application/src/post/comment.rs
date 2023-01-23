// application/src/post/comment.rs

use diesel::prelude::*;
use domain::models::{Comment, InsertableComment};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::request_models::NewComment;
use shared::response_models::CommentRespone;

pub fn create_post_comment(
    user_id: i32,
    post_id: i32,
    comment: Json<NewComment>,
) -> Created<String> {
    use domain::schema::comments;

    let comment_data = comment.into_inner();
    let comment = InsertableComment {
        content: comment_data.content,
        reply_to: comment_data.reply_to,
        user_id,
        post_id,
    };

    match diesel::insert_into(comments::table)
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
