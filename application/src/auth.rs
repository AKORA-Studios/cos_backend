use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub user_id: i32,
    pub username: String,
    pub nickname: String,
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,
}

fn secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.")
}

pub fn create_token(claims: JWTClaims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret().as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    match decode::<JWTClaims>(
        token,
        &DecodingKey::from_secret(secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(claims) => Ok(claims.claims),
        Err(e) => Err(e),
    }
}

/*
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub enum AuthError {
    JWTError(jsonwebtoken::errors::Error),
    MissingToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTClaims {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("Authorization");
        match token {
            Some(token) => {
                let token = if token.starts_with("Bearer ") {
                    token.replace("Bearer ", "")
                } else {
                    token.to_owned()
                };

                match verify_token(&token) {
                    Ok(claim) => Outcome::Success(claim),
                    Err(e) => Outcome::Failure((Status::Unauthorized, AuthError::JWTError(e))),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, AuthError::MissingToken)),
        }
    }
}
 */
