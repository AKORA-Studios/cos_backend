use diesel::result::Error as DieselError;
use rocket::response::status::NotFound;
use shared::response_models::MessageResponse;

pub fn map_diesel_result<T>(result: Result<T, DieselError>) -> Result<T, NotFound<String>> {
    match result {
        Ok(post) => Ok(post),
        Err(err) => match err {
            DieselError::NotFound => {
                let response = MessageResponse {
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
