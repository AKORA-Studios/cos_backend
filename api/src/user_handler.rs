// api/src/post_handler.rs

use application::user::{login, read, register};
use rocket::response::status::{Created, NotFound, Unauthorized};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::request_models::{LoginCredentials, RegisterUser};
use shared::response_models::UserResponse;

#[get("/users/<user_id>")]
pub fn view_user_handler(user_id: i32) -> Result<String, NotFound<String>> {
    let user = read::view_user(user_id)?;
    let response = UserResponse { user };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/login", format = "application/json", data = "<credentials>")]
pub fn login_user_handler(
    credentials: Json<LoginCredentials>,
) -> Result<std::string::String, Unauthorized<String>> {
    login::login_user(credentials)
}

#[post("/register", format = "application/json", data = "<user>")]
pub fn register_user_handler(user: Json<RegisterUser>) -> Created<String> {
    register::register_user(user)
}
