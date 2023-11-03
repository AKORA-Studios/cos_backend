use std::convert;
use std::ops::{ControlFlow, Deref, FromResidual, Try};

use application::{OpErr, OpResult, OpSuc};
use axum::http::response::Builder;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub struct WrappedRes<V: Serialize, E: Serialize>(pub OpResult<V, E>);
impl<V: Serialize, E: Serialize> From<OpResult<V, E>> for WrappedRes<V, E> {
    fn from(value: OpResult<V, E>) -> Self {
        WrappedRes(value)
    }
}

impl<V: Serialize, E: Serialize> Deref for WrappedRes<V, E> {
    type Target = OpResult<V, E>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V: Serialize, E: Serialize> FromResidual for WrappedRes<V, E> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        WrappedRes(Err(residual))
    }
}

impl<V: Serialize, E: Serialize> Try for WrappedRes<V, E> {
    type Output = OpSuc<V>;
    type Residual = OpErr<E>;

    // Required methods
    fn from_output(output: Self::Output) -> Self {
        WrappedRes(Ok(output))
    }
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(e),
        }
    }
}

impl<V: Serialize, E: Serialize> IntoResponse for WrappedRes<V, E> {
    fn into_response(self) -> Response {
        let mut response_builder = Builder::new();
        let response = match self.0 {
            Ok(suc) => {
                let (status, body) = match suc {
                    OpSuc::Created(v) => (StatusCode::CREATED, v),
                    OpSuc::Success(v) => (StatusCode::OK, v),
                    OpSuc::Deleted(v) => (StatusCode::NO_CONTENT, v),
                    OpSuc::Read(v) => (StatusCode::OK, v),
                    OpSuc::Updated(v) => (StatusCode::OK, v),
                };

                (status, Json(body)).into_response()
            }
            Err(err) => {
                let (status, body) = match err {
                    OpErr::Any(v) => (StatusCode::NOT_FOUND, v),
                    OpErr::InternalError(v) => (StatusCode::NOT_FOUND, v),
                    OpErr::Unauthorized(v) => (StatusCode::NOT_FOUND, v),
                    OpErr::NotFound(v) => (StatusCode::NOT_FOUND, v),
                };

                (status, Json(body)).into_response()
            }
        };

        response
    }
}
