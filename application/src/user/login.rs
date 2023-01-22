// application/src/user/create.rs

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use domain::models::User;
use infrastructure::establish_connection;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use shared::{
    request_models::LoginCredentials,
    response_models::{ErrorMessageResponse, TokenRespone},
};
use std::time::{Duration, SystemTime};

use crate::auth::{self, JWTClaims};

fn unauthorized<T>() -> Result<T, Unauthorized<String>> {
    let response = ErrorMessageResponse {
        message: format!("Invalid password or username"),
    };

    Err(Unauthorized(Some(
        serde_json::to_string(&response).unwrap(),
    )))
}

pub fn login_user(credentials: Json<LoginCredentials>) -> Result<String, Unauthorized<String>> {
    use domain::schema::users;
    let creds = credentials.into_inner();

    let (password, result) = match creds {
        LoginCredentials::UsernameCredentials { username, password } => (
            password,
            users::table
                .filter(users::username.eq(username))
                .first::<User>(&mut establish_connection()),
        ),
        LoginCredentials::EmailCredentials { email, password } => (
            password,
            users::table
                .filter(users::email.eq(email))
                .first::<User>(&mut establish_connection()),
        ),
    };

    match result {
        Ok(user) => match PasswordHash::new(&user.password_hash) {
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

                                Ok(serde_json::to_string(&response).unwrap())
                            }
                            Err(e) => panic!("JWT encoding error - {}", e),
                        }
                    }
                    Err(_e) => unauthorized(),
                }
            }
            Err(e) => panic!("Password hashing error - {}", e),
        },
        Err(err) => match err {
            DieselError::NotFound => unauthorized(),
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
