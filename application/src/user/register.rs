// application/src/user/create.rs

use domain::models::{DisplayUser, DISPLAY_USER_COLUMNS};
use shared::{request_models::RegisterUser, response_models::UserResponse};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::PgPool;

use crate::{map_sqlx_result, OpResult, OpSuc};

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

pub async fn register_user(conn: &PgPool, user: RegisterUser) -> OpResult<UserResponse, String> {
    let hashed_password = hash_password(user.password.as_bytes());

    let sql = format!(
        r#"
        INSERT INTO users
        (username, nickname, password_hash, email, twitter_username, instagram_username, tiktok_username, onlyfans_username, snapchat_username, youtube_username, myanimelist_username)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING {DISPLAY_USER_COLUMNS}
    "#
    );
    let result = sqlx::query_as::<_, DisplayUser>(&sql)
        .bind(user.username)
        .bind(user.nickname)
        .bind(hashed_password)
        .bind(user.email)
        //
        .bind(user.twitter_username)
        .bind(user.instagram_username)
        .bind(user.tiktok_username)
        .bind(user.onlyfans_username)
        .bind(user.snapchat_username)
        .bind(user.youtube_username)
        .bind(user.myanimelist_username)
        .fetch_one(conn)
        .await;

    map_sqlx_result(result.map(|v| OpSuc::Created(UserResponse { user: v })))
}
