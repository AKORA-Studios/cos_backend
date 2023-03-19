// api/src/image_handler.rs

use std::path::PathBuf;

use application::auth::JWTClaims;
use rocket::data::ByteUnit;
use rocket::http::Status;
use rocket::tokio::fs::File;
use rocket::{get, put, Data};

const UPLOAD_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../images");

fn profile_picture_path(user_id: i32) -> PathBuf {
    PathBuf::from(format!("{UPLOAD_DIR}/users/{user_id}"))
}

fn post_picture_path(post_id: i32, image: u32) -> PathBuf {
    PathBuf::from(format!("{UPLOAD_DIR}/posts/{post_id}/{image}"))
}

// Handle user images
#[put("/users/<user_id>", data = "<image>")]
pub async fn upload_profile_picture_handler(
    user: JWTClaims,
    user_id: i32,
    image: Data<'_>,
) -> std::io::Result<Status> {
    if user.user_id != user_id {
        return Ok(Status::Forbidden);
    }

    image
        .open(ByteUnit::Megabyte(10))
        .into_file(profile_picture_path(user_id))
        .await?;

    Ok(Status::Ok)
}

#[get("/users/<user_id>")]
pub async fn retrieve_profile_picture_handler(user_id: i32) -> Option<File> {
    let filename = profile_picture_path(user_id);
    File::open(&filename).await.ok()
}

// Handle post images
#[put("/posts/<_post_id>/<_image_count>", data = "<_image>")]
pub async fn upload_post_picture_handler(
    _user: JWTClaims,
    _post_id: i32,
    _image_count: u32,
    _image: Data<'_>,
) -> std::io::Result<Status> {
    // !TODO implement checks for post owner

    return Ok(Status::Forbidden);
}

#[get("/users/<post_id>/<image_count>")]
pub async fn retrieve_post_picture_handler(post_id: i32, image_count: u32) -> Option<File> {
    let filename = post_picture_path(post_id, image_count);
    File::open(&filename).await.ok()
}
