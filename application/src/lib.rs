// application/src/lib.rs

pub mod auth;
// pub mod event;
// pub mod message;
// pub mod post;
pub mod user;
mod util;
pub use util::map_sqlx_result;
pub use util::OperationError as OpErr;
pub use util::OperationResult as OpResult;
pub use util::OperationSuccess as OpSuc;
