use std::ops::Deref;

use application::{
    auth::{verify_token, JWTClaims},
    user::login,
    OpResult, OpSuc,
};
use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt, TypedHeader,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::{request_models::LoginCredentials, response_models::TokenRespone};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub async fn login_user_handler(
    State(pool): State<PgPool>,
    Json(credentials): Json<LoginCredentials>,
) -> OpResult<TokenRespone, String> {
    let (password, user) = login::fetch_user_with_credentials(&pool, credentials).await;

    if let OpSuc::Read(user) = user? {
        login::authorize_user(&password, user).await
    } else {
        unreachable!("Invalid operation")
    }
}

async fn protected(claims: JWTClaims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

pub struct Claims(pub JWTClaims);
impl Deref for Claims {
    type Target = JWTClaims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = verify_token(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

        Ok(Claims(token_data))
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
