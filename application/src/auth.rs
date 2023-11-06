use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use once_cell::sync::Lazy;
use shared::auth::JWTClaims;

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

pub fn create_token(claims: JWTClaims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &claims, &KEYS.encoding)
}

pub fn verify_token(token: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    match decode::<JWTClaims>(token, &KEYS.decoding, &Validation::new(Algorithm::HS256)) {
        Ok(claims) => Ok(claims.claims),
        Err(e) => Err(e),
    }
}
