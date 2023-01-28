// application/src/user/read.rs

use diesel::prelude::*;
use domain::models::{DisplayUser, PatchedUser, DISPLAY_USER_COLUMNS};
use infrastructure::establish_connection;
use rocket::{response::status::NotFound, serde::json::Json};

use crate::util::map_diesel_result;

pub fn modify_user(
    user_id: i32,
    patch_data: Json<PatchedUser>,
) -> Result<DisplayUser, NotFound<String>> {
    use domain::schema::users::dsl::*;

    let patch_data = patch_data.into_inner();

    let statement = diesel::update(users.filter(id.eq(user_id))).set(patch_data);

    let result = statement
        .returning(DISPLAY_USER_COLUMNS)
        .get_result::<DisplayUser>(&mut establish_connection());

    map_diesel_result(result)
}
