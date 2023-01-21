use diesel::result::Error as DieselError;
use diesel::PgConnection;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::ErrorMessageResponse;

pub fn map_diesel_result<T>(result: Result<T, DieselError>) -> Result<T, NotFound<String>> {
    match result {
        Ok(post) => Ok(post),
        Err(err) => match err {
            DieselError::NotFound => {
                let response = ErrorMessageResponse {
                    message: format!("Not found"),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

use rocket::request::{FromRequest, Outcome, Request};
pub struct DbConn(PgConnection);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = jsonwebtoken::errors::Error;

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(DbConn(establish_connection()))
    }
}
