// api/src/image_handler.rs

use std::path::PathBuf;

use application::{post, OpErr, OpResult, OpSuc};
use axum::{
    body::{boxed, Body, BoxBody, StreamBody},
    extract::{BodyStream, Path, State},
    http::{header, Request, StatusCode, Uri},
    response::{IntoResponse, Response},
};

use tower::util::ServiceExt;
use tower_http::services::ServeDir;

use futures_util::stream::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::extractors::auth::Claims;

fn get_path(join: &str) -> PathBuf {
    std::env::current_dir()
        .expect("Unable to get CWD")
        .join("contents")
        .join(join)
}

fn profile_picture_path(user_id: i32) -> PathBuf {
    get_path(format!("users/{user_id}").as_str())
}

fn post_picture_path(post_id: i32, image_id: u32) -> PathBuf {
    get_path(format!("posts/{post_id}/{image_id}").as_str())
}

/// GET /posts/<post_id>/contents
pub async fn list_post_contents(
    State(pool): State<sqlx::PgPool>,
    Path(post_id): Path<i32>,
) -> OpResult<Vec<String>, String> {
    // Check if post still exists
    let _post = post::read::view_post(&pool, post_id, None).await?;

    let picture_path = post_picture_path(post_id, 0);
    let post_dir_path = picture_path.parent().unwrap();

    let exists = tokio::fs::try_exists(post_dir_path).await?;
    if !exists {
        return Err(OpErr::NotFound("Post has no contents".to_owned()));
    }

    let mut dir = tokio::fs::read_dir(post_dir_path).await?;
    let mut contents = Vec::new();
    while let Some(entry) = dir.next_entry().await? {
        let Ok(file_name) = entry.file_name().into_string() else {
            eprintln!("String error while parsing {:?}", entry.file_name());
            return Err(OpErr::internal_error());
        };
        contents.push(format!("/contents/posts/{post_id}/{file_name}"));
    }

    Ok(OpSuc::Success(contents))
}

/// POST /posts/<post_id>/upload/<image_id>
pub async fn upload_post_picture_handler(
    State(pool): State<sqlx::PgPool>,
    Claims(claims): Claims,
    Path((post_id, image_id)): Path<(i32, u32)>,
    headers: header::HeaderMap,
    mut stream: BodyStream,
) -> OpResult<String, String> {
    if image_id > 5 {
        return Err(OpErr::BadRequest(
            "Only up to 5 pictures allowed".to_owned(),
        ));
    }

    let post = post::read::view_post(&pool, post_id, Some(claims.user_id)).await?;
    if post.author.user_id != claims.user_id {
        return Err(OpErr::Unauthorized(
            "You need to be the author of the post".to_owned(),
        ));
    }

    let Some(content_type) = headers.get(header::CONTENT_TYPE) else {
        return Err(OpErr::BadRequest(
            "Missing 'content-type' header".to_owned(),
        ));
    };

    let mut picture_path = post_picture_path(post_id, image_id);
    let Ok(content_type) = content_type.to_str() else {
        return Err(OpErr::BadRequest(
            "Invalid 'content-type' header value, header can only contain visible ASCII chars"
                .to_owned(),
        ));
    };

    let Some(extension) = mime_guess::get_mime_extensions_str(content_type).map(|v| v[0]) else {
        return Err(OpErr::BadRequest(
            "'content-type' header value is not a valid MIME type".to_owned(),
        ));
    };

    if picture_path.set_extension(extension) == false {
        // Setting the extension failed
        eprintln!("Setting file extension failed OwO");
        return Err(OpErr::internal_error());
    }

    let exists = tokio::fs::try_exists(&picture_path).await?;
    if exists {
        return Err(OpErr::BadRequest("Picture already exists".to_owned()));
    } else {
        let post_dir_path = picture_path.parent().unwrap();
        let dir_exists = tokio::fs::try_exists(post_dir_path).await?;
        if !dir_exists {
            tokio::fs::create_dir(post_dir_path).await?;
        }
    }

    let mut file = tokio::fs::File::create(&picture_path).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(OpSuc::Created(format!(
        "/contents/posts/{post_id}/{image_id}.{extension}"
    )))
}

pub async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new(get_path("")).oneshot(req).await {
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
