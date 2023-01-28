// application/src/post/create.rs

use diesel::prelude::*;
use domain::models::{InsertablePost, Post};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::request_models::NewPost;
use shared::response_models::PostResponse;

pub fn create_post(user_id: i32, post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;

    let post = post.into_inner();
    let post = InsertablePost {
        user_id,
        caption: post.caption,
        description: post.description,
        tags: post.tags,
        photographer_id: post.photographer_id,
        lat: post.lat,
        lon: post.lon,
    };

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
