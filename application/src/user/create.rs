// application/src/user/create.rs

use diesel::prelude::*;
use domain::models::{DisplayUser, NewUser, DISPLAY_USER_COLUMNS};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::UserResponse;

pub fn create_user(user: Json<NewUser>) -> Created<String> {
    use domain::schema::users::dsl::*;

    let user = user.into_inner();

    match diesel::insert_into(users)
        .values(&user)
        .returning(DISPLAY_USER_COLUMNS)
        .get_result::<DisplayUser>(&mut establish_connection())
    {
        Ok(user) => {
            let response = UserResponse { user };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
