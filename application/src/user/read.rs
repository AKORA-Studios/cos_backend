// application/src/user/read.rs

use diesel::prelude::*;
use domain::models::{DisplayUser, DISPLAY_USER_COLUMNS};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;

use crate::util::map_diesel_result;

pub fn view_user(user_id: i32) -> Result<DisplayUser, NotFound<String>> {
    use domain::schema::users::dsl::*;

    let result = users
        .select(DISPLAY_USER_COLUMNS)
        .find(user_id)
        .first::<DisplayUser>(&mut establish_connection());

    map_diesel_result(result)
}
