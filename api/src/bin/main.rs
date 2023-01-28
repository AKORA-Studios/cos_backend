// api/src/bin/main.rs
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;
use api::event_handler;
use api::message_handler;
use api::post_handler;
use api::user_handler;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build().mount(
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
}
