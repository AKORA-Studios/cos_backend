use serde::Serialize;

type Serializeable<T: Serialize> = T;
pub type OpResult<V: Serialize, E: Serialize> = Result<OperationSuccess<V>, OperationError<E>>;
type OperationResult<V: Serialize, E: Serialize> = OpResult<V, E>;

/// OperationSuccess
pub enum OpSuc<V: Serialize> {
    Success(V),
    Created(V),
    Updated(V),
    Deleted(V),
    Read(V),
}
type OperationSuccess<V: Serialize> = OpSuc<V>;

/// OperationError
pub enum OpErr<E: Serialize> {
    InternalError(E),
    Unauthorized(E),
    NotFound,
    Any,
}
type OperationError<V: Serialize> = OpErr<V>;

pub fn map_sqlx_result<T: Serialize>(
    result: Result<OperationSuccess<T>, sqlx::Error>,
) -> OperationResult<T, String> {
    result.map_err(|e| match e {
        sqlx::Error::RowNotFound => OperationError::NotFound,
        _ => OperationError::InternalError(e.to_string()),
    })
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
