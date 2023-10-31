use std::fmt::Display;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub user_id: i32,
    pub username: String,
    pub nickname: String,
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,
}

impl Display for JWTClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {}\nName: {}", self.user_id, self.username)
    }
}

pub fn create_token(claims: JWTClaims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &claims, &KEYS.encoding)
}

pub fn verify_token(token: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    match decode::<JWTClaims>(token, &KEYS.decoding, &Validation::new(Algorithm::HS256)) {
        Ok(claims) => Ok(claims.claims),
        Err(e) => Err(e),
    }
}
