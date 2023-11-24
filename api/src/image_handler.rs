// api/src/image_handler.rs

use std::path::PathBuf;

use axum::{
    body::{boxed, Body, BoxBody, StreamBody},
    http::{header, Request, StatusCode, Uri},
    response::IntoResponse,
    response::Response,
};

use tower::util::ServiceExt;
use tower_http::services::ServeDir;

use tokio::fs::File;
use tokio_util::io::ReaderStream;

const UPLOAD_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../images");

fn profile_picture_path(user_id: i32) -> PathBuf {
    PathBuf::from(format!("{UPLOAD_DIR}/users/{user_id}"))
}

fn post_picture_path(post_id: i32, image: u32) -> PathBuf {
    PathBuf::from(format!("{UPLOAD_DIR}/posts/{post_id}/{image}"))
}

pub async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../contents"))
        .oneshot(req)
        .await
    {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

/// Update user image
/// PUT /users/me       <image>
/*
pub async fn upload_profile_picture_handler(
    Claims(claims): Claims,
    mut stream: BodyStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = profile_picture_path(claims.user_id);
    let mut file = File::open(path).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}
*/

/// get /users/<user_id>/image
pub async fn retrieve_profile_picture_handler(user_id: i32) -> Response {
    let filename = profile_picture_path(user_id);
    let file = if let Ok(f) = File::open(&filename).await {
        f
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "mhm").into_response();
    };

    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let headers = [
        (header::CONTENT_TYPE, "image/png"),
        (
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{user_id}.png\""),
        ),
    ];

    (headers, body).into_response()
}

/// Handle post images
/// PUT /posts/:post_id>/:image_count        <image>
/*
pub async fn upload_post_picture_handler(
    _user: JWTClaims,
    _post_id: i32,
    _image_count: u32,
    _image: Data<'_>,
) -> std::io::Result<Status> {
    // !TODO implement checks for post owner

    return Ok(Status::Forbidden);
}
*/

/// get /users/<post_id>/<image_count>
pub async fn retrieve_post_picture_handler(post_id: i32, image_count: u32) -> Option<File> {
    let filename = post_picture_path(post_id, image_count);
    File::open(&filename).await.ok()
}
