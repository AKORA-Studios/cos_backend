// api/src/bin/main.rs

#[macro_use]
extern crate rocket;
use api::event_handler;
use api::message_handler;
use api::post_handler;
use api::user_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            post_handler::list_today_posts_handler,
            post_handler::list_recent_posts_handler,
            post_handler::list_user_posts_handler,
            post_handler::create_post_handler,
            post_handler::create_comment_handler,
            post_handler::list_recent_comments_handler,
            user_handler::create_user_handler,
            user_handler::view_user_handler,
            event_handler::create_event_handler,
            event_handler::view_event_handler,
            message_handler::create_message_handler,
            message_handler::view_message_handler,
        ],
    )
}
