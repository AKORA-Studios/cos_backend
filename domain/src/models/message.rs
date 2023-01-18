// domain/src/models.rs
use diesel::prelude::*;

use super::user::User;
use crate::schema::{attachments, messages};
use crate::sql_types::ContentType;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::time::SystemTime;

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Identifiable, Queryable, Serialize, Associations, PartialEq, Eq, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(User, foreign_key = to_id))]
#[diesel(table_name = messages)]
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
#[derive(Identifiable, Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = attachments)]
#[diesel(primary_key(id))]
pub struct Attachment {
    pub id: i32,
    pub url: String,
    pub content_type: ContentType,
    pub created_at: SystemTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub content: String,
    pub from_id: i32,
    pub to_id: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = attachments)]
pub struct NewAttachment {
    pub url: String,
    pub content_type: ContentType,
    pub created_at: SystemTime,
}
