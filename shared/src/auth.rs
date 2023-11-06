use std::fmt::Display;

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

impl Display for JWTClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {}\nName: {}", self.user_id, self.username)
    }
}
