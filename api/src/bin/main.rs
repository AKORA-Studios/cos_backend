use api::extractors::auth::login_user_handler;
// api/src/bin/main.rs
use dotenvy::dotenv;
use std::{env, time::Duration};
use tokio;

//event_handler, image_handler, message_handler, post_handler,
use std::fs;

use axum::{
    http::StatusCode,
    routing::{delete, get, patch, post, put},
    Router,
};
use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    // Create image directories if missing
    {
        let upload_dir = env::current_dir()
            .expect("Unable to get CWD")
            .join("contents");

        fs::create_dir_all(upload_dir.join("users"))
            .expect("Unable to create user contents upload dir");
        fs::create_dir_all(upload_dir.join("posts"))
            .expect("Unable to create post contents upload dir");
        fs::create_dir_all(upload_dir.join("events"))
            .expect("Unable to create event contents upload dir");
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // Create all preapred statements
                application::prepare_statements(conn).await?;

                Ok(())
            })
        })
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    if let Err(e) = infrastructure::run_migrations(&pool).await {
        panic!("Error while running migrations: {e}")
    }

    use api::event_handler::*;
    use api::message_handler::*;
    use api::post_handler::*;
    use api::user_handler::*;

    // build our application with a route
    let user_router = Router::new()
        .route("/users/:user_id", get(view_user_handler))
        .route("/users/:user_id/posts", get(list_user_posts_handler))
        .route("/users/:user_id/follow", put(follow_user_handler))
        .route("/users/:user_id/unfollow", put(unfollow_user_handler))
        .route("/users/:user_id/block", put(block_user_handler))
        .route("/users/:user_id/unblock", put(unblock_user_handler))
        .route("/users/me", get(view_me_handler))
        .route("/users/me", patch(patch_me_handler));

    let post_router = Router::new()
        .route("/posts/new", post(create_post_handler))
        .route("/posts/:post_id", get(view_post_handler))
        .route("/posts/:post_id", delete(delete_post_handler))
        .route("/posts/:post_id/like", put(like_post_handler))
        .route("/posts/:post_id/dislike", put(dislike_post_handler))
        .route("/posts/:post_id/download", put(download_post_handler))
        .route("/posts/:post_id/comments/new", post(create_comment_handler))
        .route(
            "/posts/:post_id/comments/recent", //?limit",
            get(list_recent_comments_handler),
        )
        .route("/posts/recent", get(list_recent_posts_handler));

    let event_router = Router::new()
        .route("/events/new", post(create_event_handler))
        .route("/events/:event_id", get(view_event_handler));

    let message_router = Router::new()
        .route("/users/:user_id/messages/new", post(create_message_handler))
        .route("/users/:user_id/messages", get(list_conversation_handler));

    // Actual app creation

    let app = Router::new()
        .route("/", get(status_handler))
        .nest(
            "/contents",
            Router::new().fallback(get(api::image_handler::get_static_file)),
        )
        .nest(
            "/api",
            Router::new()
                .route("/login", post(login_user_handler))
                .route("/register", post(register_user_handler))
                .merge(user_router)
                .merge(post_router)
                .merge(event_router)
                .merge(message_router),
        )
        .fallback(fallback_handler)
        .with_state(pool);

    // Allow port configuratrion through environment variables
    let port = env::var("PORT")
        .unwrap_or("3000".to_owned())
        .parse()
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    //tracing::debug!("listening on {}", addr);
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn status_handler() -> StatusCode {
    StatusCode::OK
}

async fn fallback_handler() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "This route, doesn't exist".to_owned(),
    )
}
