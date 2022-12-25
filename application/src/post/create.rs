// application/src/post/create.rs

use diesel::prelude::*;
use domain::models::{NewPost, Post};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn create_post(post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;

    let post = post.into_inner();

    match diesel::insert_into(posts::table)
        .values(&post)
        .get_result::<Post>(&mut establish_connection())
    {
        Ok(post) => {
            let response = Response {
                body: ResponseBody::Post(post),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
