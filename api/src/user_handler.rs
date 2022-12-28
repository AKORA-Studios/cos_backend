// api/src/post_handler.rs

use application::user::create;
use domain::models::NewUser;
use rocket::post;
use rocket::response::status::Created;
use rocket::serde::json::Json;

#[post("/users/new", format = "application/json", data = "<user>")]
pub fn create_user_handler(user: Json<NewUser>) -> Created<String> {
    create::create_user(user)
}
