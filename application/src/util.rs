use shared::response_models::ErrorMessageResponse;

pub type OperationResult<V, E> = Result<OperationSucces<V>, OperationError<E>>;

pub enum OperationSucces<V> {
    Created(V),
    Updated(V),
    Deleted(V),
    Read(V),
}
pub enum OperationError<E> {
    InternalError(E),
    NotFound(E),
}

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
