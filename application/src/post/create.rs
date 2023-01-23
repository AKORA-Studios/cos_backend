// application/src/post/create.rs

use diesel::prelude::*;
use domain::models::{InsertablePost, Post};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::PostResponse;

pub fn create_post(post: Json<InsertablePost>) -> Created<String> {
    use domain::schema::posts;

    let post = post.into_inner();

    match diesel::insert_into(posts::table)
        .values(&post)
        .get_result::<Post>(&mut establish_connection())
    {
        Ok(post) => {
            let response = PostResponse { post };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
