use axum::{
    async_trait,
    extract::{rejection::QueryRejection, FromRequestParts, Query},
    http::request::Parts,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryObject {
    limit: Option<i64>,
}

/// Extracts the often used `limit` query parameter and automatically limits it to the range of 0-50
pub struct Limit(pub Option<i64>);

#[async_trait]
impl<S> FromRequestParts<S> for Limit
where
    S: Send + Sync,
{
    type Rejection = QueryRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Query::<QueryObject>::from_request_parts(parts, state)
            .await
            .map(|q| Limit(q.0.limit.map(|l| l.abs().max(50))))
    }
}
