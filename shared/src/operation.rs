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
    BadRequest(E),
    Any(E),
}

impl OpErr<String> {
    pub fn internal_error() -> Self {
        Self::InternalError("Internal Server error".to_owned())
    }
}

impl From<std::io::Error> for OpErr<String> {
    fn from(value: std::io::Error) -> Self {
        use std::io::ErrorKind;

        match value.kind() {
            ErrorKind::NotFound => OpErr::NotFound("File not found".to_owned()),
            _ => {
                eprintln!("{value:?}");
                OpErr::Any("IO error".to_owned())
            }
        }
    }
}

impl From<axum::Error> for OpErr<String> {
    fn from(value: axum::Error) -> Self {
        eprintln!("{value:?}");

        OpErr::internal_error()
    }
}

impl From<sqlx::Error> for OpErr<String> {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => OpErr::NotFound("".to_owned()),
            _ => {
                eprintln!("{value:?}");
                OpErr::internal_error()
            }
        }
    }
}

type OperationError<V> = OpErr<V>;

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
            OpErr::Any(v) => (StatusCode::INTERNAL_SERVER_ERROR, v),
            OpErr::InternalError(v) => (StatusCode::INTERNAL_SERVER_ERROR, v),
            OpErr::Unauthorized(v) => (StatusCode::UNAUTHORIZED, v),
            OpErr::NotFound(v) => (StatusCode::NOT_FOUND, v),
            OpErr::BadRequest(v) => (StatusCode::BAD_REQUEST, v),
        };

        (status, Json(body)).into_response()
    }
}
