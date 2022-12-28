// api/src/post_handler.rs

use application::user::{create, read};
use domain::models::NewUser;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::UserResponse;

#[post("/users/new", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>) -> Created<String> {
    create::create_user(user)
}

#[get("/users/<user_id>")]
pub fn view_user_handler(user_id: i32) -> Result<String, NotFound<String>> {
    let user = read::view_user(user_id)?;
    let response = UserResponse { user };

    Ok(serde_json::to_string(&response).unwrap())
}
