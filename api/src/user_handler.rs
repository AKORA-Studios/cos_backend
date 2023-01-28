// api/src/post_handler.rs

use application::auth::JWTClaims;
use application::user::{interact, login, read, register};
use rocket::response::status::{Created, NotFound, Unauthorized};
use rocket::serde::json::Json;
use rocket::{get, post, put};
use shared::request_models::{LoginCredentials, RegisterUser};
use shared::response_models::UserResponse;

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

#[get("/users/<user_id>")]
pub fn view_user_handler(user_id: i32) -> Result<String, NotFound<String>> {
    let user = read::view_user(user_id)?;
    let response = UserResponse { user };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/users/<user_id>/follow")]
pub fn follow_user_handler(user: JWTClaims, user_id: i32) -> Result<(), NotFound<String>> {
    interact::follow_user(user.user_id, user_id)
}

#[put("/users/<user_id>/unfollow")]
pub fn unfollow_user_handler(user: JWTClaims, user_id: i32) -> Result<(), NotFound<String>> {
    interact::unfollow_user(user.user_id, user_id)
}

#[put("/users/<user_id>/block")]
pub fn block_user_handler(user: JWTClaims, user_id: i32) -> Result<(), NotFound<String>> {
    interact::block_user(user.user_id, user_id)
}

#[put("/users/<user_id>/unblock")]
pub fn unblock_user_handler(user: JWTClaims, user_id: i32) -> Result<(), NotFound<String>> {
    interact::unblock_user(user.user_id, user_id)
}
