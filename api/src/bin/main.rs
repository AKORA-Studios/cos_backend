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
        let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../images");
        fs::create_dir_all(format!("{upload_dir}/users"))
            .expect("Unable to create user images upload dir");
        fs::create_dir_all(format!("{upload_dir}/posts"))
            .expect("Unable to create post images upload dir");
        fs::create_dir_all(format!("{upload_dir}/events"))
            .expect("Unable to create event images upload dir");
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
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
    let app = Router::new()
        .route("/", get(status_handler))
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
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
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

/*
fn rocket() -> _ {
    let db: Map<_, Value> = map! {
        "url" => database_url.into(),
        "pool_size" => 10.into(),
        "timeout" => 5.into(),
    };

    let figment = rocket::Config::figment().merge(("databases", map!["cos" => db]));

    rocket::custom(figment)
        .attach(DbConn::fairing())
        .mount(
            "/api",
            routes![
                // POST
                post_handler::view_post_handler,
                post_handler::list_today_posts_handler,
                post_handler::list_recent_posts_handler,
                post_handler::list_user_posts_handler,
                post_handler::create_post_handler,
                post_handler::create_comment_handler,
                post_handler::list_recent_comments_handler,
                // INTERACT
                post_handler::delete_post_handler,
                post_handler::like_post_handler,
                post_handler::dislike_post_handler,
                post_handler::download_post_handler,
                // USER
                user_handler::register_user_handler,
                user_handler::login_user_handler,
                user_handler::view_user_handler,
                user_handler::view_me_handler,
                user_handler::patch_me_handler,
                // INTERACT
                user_handler::follow_user_handler,
                user_handler::unfollow_user_handler,
                user_handler::block_user_handler,
                user_handler::unblock_user_handler,
                // EVENT
                event_handler::create_event_handler,
                event_handler::view_event_handler,
                // MESSAGE
                message_handler::create_message_handler,
                message_handler::list_conversation_handler,
            ],
        )
        .mount(
            "/images",
            routes![
                image_handler::upload_profile_picture_handler,
                image_handler::retrieve_profile_picture_handler,
                image_handler::upload_post_picture_handler,
                image_handler::retrieve_post_picture_handler,
            ],
        )
}
 */
