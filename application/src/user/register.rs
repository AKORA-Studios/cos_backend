// application/src/user/create.rs

use diesel::prelude::*;
use domain::models::{DisplayUser, NewUser, DISPLAY_USER_COLUMNS};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::{request_models::RegisterUser, response_models::UserResponse};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    argon2
        .hash_password(password, &salt)
        .expect("Unable to hash password")
        .to_string()
}

pub fn register_user(user: Json<RegisterUser>) -> Created<String> {
    use domain::schema::users::dsl::*;

    let user = user.into_inner();
    let hashed_password = hash_password(user.password.as_bytes());
    let user = NewUser {
        username: user.username,
        nickname: user.nickname,
        password_hash: hashed_password,
        email: user.email,

        twitter_username: user.twitter_username,
        instagram_username: user.instagram_username,
        tiktok_username: user.tiktok_username,
        onlyfans_username: user.onlyfans_username,
        snapchat_username: user.snapchat_username,
        youtube_username: user.youtube_username,
        myanimelist_username: user.myanimelist_username,
    };

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
