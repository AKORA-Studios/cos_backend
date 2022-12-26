// api/src/post_handler.rs

use application::post::{create, read};
use domain::models::NewPost;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::{Response, ResponseBody};

#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    // 👇 New function body!
    let post = read::list_post(post_id)?;
    let response = Response {
        body: ResponseBody::Post(post),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/user/<user_id>/posts?<limit>")]
pub fn list_posts_handler(user_id: i32, limit: usize) -> Result<String, NotFound<String>> {
    // 👇 New function body!
    let posts = read::list_posts(user_id, limit);
    let response = Response {
        body: ResponseBody::Posts(posts),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_post", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<NewPost>) -> Created<String> {
    create::create_post(post)
}
