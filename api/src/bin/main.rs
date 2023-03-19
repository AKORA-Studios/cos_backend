// api/src/bin/main.rs
use dotenvy::dotenv;
use std::env;

#[macro_use]
extern crate rocket;
use api::{event_handler, image_handler, message_handler, post_handler, user_handler};
use infrastructure::DbConn;
use rocket::figment::map;
use rocket::figment::value::{Map, Value};

use std::fs;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let db: Map<_, Value> = map! {
        "url" => database_url.into(),
        "pool_size" => 10.into(),
        "timeout" => 5.into(),
    };

    let figment = rocket::Config::figment().merge(("databases", map!["cos" => db]));

    // Create image directories if missing
    {
        let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "../images");
        fs::create_dir_all(format!("{upload_dir}/users"))
            .expect("Unable to create user images upload dir");
        fs::create_dir_all(format!("{upload_dir}/posts"))
            .expect("Unable to create post images upload dir");
        fs::create_dir_all(format!("{upload_dir}/events"))
            .expect("Unable to create event images upload dir");
    }

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
