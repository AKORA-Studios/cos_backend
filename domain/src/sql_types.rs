use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

// In sql_types.rs
#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
#[DieselTypePath = "crate::schema::sql_types::ContentType"]
pub enum ContentType {
    Image,
    Video,
    Audio,
}
