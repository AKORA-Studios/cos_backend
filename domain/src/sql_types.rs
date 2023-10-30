use serde::{Deserialize, Serialize};

// In sql_types.rs
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContentType {
    Image,
    Video,
    Audio,
}
