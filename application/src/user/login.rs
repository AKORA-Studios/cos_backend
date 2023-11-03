// application/src/user/create.rs
use domain::models::User;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::Serialize;
use shared::{
    request_models::LoginCredentials,
    response_models::{ErrorMessageResponse, TokenRespone},
};
use sqlx::PgPool;
use std::time::{Duration, SystemTime};

use crate::{
    auth::{self, JWTClaims},
    map_sqlx_result,
};

use crate::{OpErr, TaskResult};

fn unauthorized<T: Serialize>() -> TaskResult<T, String> {
    let response = ErrorMessageResponse {
        message: format!("Invalid password or username"),
    };

    Err(OpErr::Unauthorized(
        serde_json::to_string(&response).unwrap(),
    ))
}

pub async fn fetch_user_with_credentials(
    conn: &PgPool,
    credentials: LoginCredentials,
) -> (String, TaskResult<User, String>) {
    let (password, user) = match credentials {
        LoginCredentials::UsernameCredentials { username, password } => (
            password,
            sqlx::query_as::<_, User>(r#"SELECT * FROM "users" WHERE username = $1"#)
                .bind(username)
                .fetch_one(conn)
                .await,
        ),
        LoginCredentials::EmailCredentials { email, password } => (
            password,
            sqlx::query_as::<_, User>(r#"SELECT * FROM "users" WHERE email = $1"#)
                .bind(email)
                .fetch_one(conn)
                .await,
        ),
    };

    (password, map_sqlx_result(user))
}

pub async fn authorize_user(password: &str, user: User) -> TaskResult<TokenRespone, String> {
    match PasswordHash::new(&user.password_hash) {
        Ok(parsed_hash) => {
            match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                Ok(_) => {
                    let issued_at = SystemTime::now();
                    let expires_at = SystemTime::now() + Duration::from_secs(60 * 60 * 8);

                    let iat = issued_at
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let exp = expires_at
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let claims = JWTClaims {
                        user_id: user.id,
                        username: user.username,
                        nickname: user.nickname,
                        exp: exp as usize,
                        iat: iat as usize,
                    };

                    match auth::create_token(claims) {
                        Ok(token) => {
                            let response = TokenRespone { token };

                            Ok(response)
                        }
                        Err(e) => panic!("JWT encoding error - {}", e),
                    }
                }
                Err(_e) => unauthorized(),
            }
        }
        Err(e) => panic!("Password hashing error - {}", e),
    }
}
