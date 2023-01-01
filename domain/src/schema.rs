// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "content_type"))]
    pub struct ContentType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ContentType;

    attachments (id) {
        id -> Int4,
        url -> Varchar,
        content_type -> ContentType,
        created_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Varchar,
        user_id -> Int4,
        post_id -> Int4,
        reply_to -> Nullable<Int4>,
        upvotes -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    depicted_people (post_id, user_id) {
        post_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    downloads (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        start_time -> Timestamp,
        end_time -> Timestamp,
        lat -> Float8,
        lon -> Float8,
    }
}

diesel::table! {
    followers (user_id, follower_id) {
        user_id -> Int4,
        follower_id -> Int4,
    }
}

diesel::table! {
    likes (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        content -> Varchar,
        attachment_id -> Int4,
        from_id -> Int4,
        to_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        caption -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        user_id -> Int4,
        tags -> Array<Nullable<Text>>,
        photographer_id -> Nullable<Int4>,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        nickname -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        twitter_username -> Nullable<Varchar>,
        instagram_username -> Nullable<Varchar>,
        tiktok_username -> Nullable<Varchar>,
        onlyfans_username -> Nullable<Varchar>,
        snapchat_username -> Nullable<Varchar>,
        youtube_username -> Nullable<Varchar>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(depicted_people -> posts (post_id));
diesel::joinable!(depicted_people -> users (user_id));
diesel::joinable!(downloads -> posts (post_id));
diesel::joinable!(downloads -> users (user_id));
diesel::joinable!(likes -> posts (post_id));
diesel::joinable!(likes -> users (user_id));
diesel::joinable!(messages -> attachments (attachment_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    comments,
    depicted_people,
    downloads,
    events,
    followers,
    likes,
    messages,
    posts,
    users,
);
