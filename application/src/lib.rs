// application/src/lib.rs

pub mod auth;
pub mod event;
pub mod message;
pub mod post;
pub mod user;
pub mod util;
pub use util::OperationError as OpErr;
pub use util::OperationResult as OpResult;
