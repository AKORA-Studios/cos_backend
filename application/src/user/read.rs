// application/src/post/read.rs

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use domain::models::User;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::MessageResponse;

pub fn view_user(user_id: i32) -> Result<User, NotFound<String>> {
    use domain::schema::users::dsl::*;

    match users
        .find(user_id)
        .first::<User>(&mut establish_connection())
    {
        Ok(post) => Ok(post),
        Err(err) => match err {
            DieselError::NotFound => {
                let response = MessageResponse {
                    message: format!("Error selecting post with id {} - {}", user_id, err),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
