use serde::Serialize;

/// Things like looking up a user are considered a Task
/// tasks don't include they're operation type in their
/// signature to make working with them easier
pub type TaskResult<V, E> = Result<V, OpErr<E>>;

/// Handling an incoming request from the
/// Server is considered a single Operation
/// containing multiple tasks
pub type OpResult<V, E> = Result<OperationSuccess<V>, OperationError<E>>;
// type OperationResult<V, E> = OpResult<V, E>;

/// OperationSuccess
pub enum OpSuc<V: Serialize> {
    Success(V),
    Created(V),
    Updated(V),
    Deleted(V),
    Read(V),
}
type OperationSuccess<V> = OpSuc<V>;

/// OperationError
pub enum OpErr<E: Serialize> {
    InternalError(E),
    Unauthorized(E),
    NotFound(E),
    Any(E),
}
type OperationError<V> = OpErr<V>;

pub fn map_sqlx_result<T>(result: Result<T, sqlx::Error>) -> Result<T, OperationError<String>> {
    result.map_err(|e| match e {
        sqlx::Error::RowNotFound => OperationError::NotFound("".to_owned()),
        _ => OperationError::InternalError(e.to_string()),
    })
}

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

impl<V: Serialize> IntoResponse for OpSuc<V> {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            OpSuc::Created(v) => (StatusCode::CREATED, v),
            OpSuc::Success(v) => (StatusCode::OK, v),
            OpSuc::Deleted(v) => (StatusCode::NO_CONTENT, v),
            OpSuc::Read(v) => (StatusCode::OK, v),
            OpSuc::Updated(v) => (StatusCode::OK, v),
        };

        (status, Json(body)).into_response()
    }
}

impl<E: Serialize> IntoResponse for OpErr<E> {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            OpErr::Any(v) => (StatusCode::NOT_FOUND, v),
            OpErr::InternalError(v) => (StatusCode::NOT_FOUND, v),
            OpErr::Unauthorized(v) => (StatusCode::NOT_FOUND, v),
            OpErr::NotFound(v) => (StatusCode::NOT_FOUND, v),
        };

        (status, Json(body)).into_response()
    }
}
