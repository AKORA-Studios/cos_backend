// application/src/lib.rs

pub mod auth;
// pub mod event;
// pub mod message;
// pub mod post;
pub mod user;
mod util;
pub use util::map_sqlx_result;
pub use util::{OpErr, OpResult, OpSuc, TaskResult};
