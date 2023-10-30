// domain/src/models.rs
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

use crate::sql_types::ContentType;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(sqlx::FromRow, Serialize, PartialEq, Eq, Debug)]
pub struct Message {
    pub id: i32,
    pub content: String,
    /// ID of the attachment if present
    pub attachment_id: Option<i32>,
    /// ID of the message this is a reply to
    pub reply_to: Option<i32>,
    /// User ID of message author
    pub from_id: i32,
    /// User ID of message receiver
    pub to_id: i32,
    pub created_at: SystemTime,
}

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Attachment {
    pub id: i32,
    pub url: String,
    pub content_type: ContentType,
    pub created_at: SystemTime,
}

#[derive(Deserialize)]
pub struct InsertableMessage {
    pub content: String,
    pub attachment_id: Option<i32>,
    pub reply_to: Option<i32>,
    pub from_id: i32,
    pub to_id: i32,
}

#[derive(Deserialize)]
pub struct InsertableAttachment {
    pub url: String,
    pub content_type: ContentType,
    pub created_at: SystemTime,
}
