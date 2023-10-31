pub type OperationResult<V, E> = Result<OperationSuccess<V>, OperationError<E>>;

pub enum OperationSuccess<V> {
    Success(V),
    Created(V),
    Updated(V),
    Deleted(V),
    Read(V),
}
pub enum OperationError<E> {
    InternalError(E),
    Unauthorized(E),
    NotFound(E),
    Any,
}

pub fn map_sqlx_result<T>(
    result: Result<OperationSuccess<T>, sqlx::Error>,
) -> OperationResult<T, sqlx::Error> {
    result.map_err(|e| OperationError::InternalError(e))
}

/*
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
*/
