// api/src/bin/main.rs

#[macro_use]
extern crate rocket;
use api::post_handler;
use api::user_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            post_handler::list_today_handler,
            post_handler::list_posts_handler,
            post_handler::list_post_handler,
            post_handler::create_post_handler,
            user_handler::create_user_handler,
        ],
    )
}
