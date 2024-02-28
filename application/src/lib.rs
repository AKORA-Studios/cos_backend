// application/src/lib.rs

pub mod auth;
pub mod event;
pub mod message;
pub mod post;
pub mod user;
mod util;
pub use util::map_sqlx_result;
pub use util::{OpErr, OpResult, OpSuc, TaskResult};

use crate::post::read::prepare_post_statements;

#[must_use = "This needs to be run when the connection is created, otherwise the queries won't run"]
pub async fn prepare_statements(conn: &mut sqlx::PgConnection) -> Result<(), sqlx::Error> {
    use futures::try_join;
    let _ = try_join!(prepare_post_statements(conn))?;

    Ok(())
}
