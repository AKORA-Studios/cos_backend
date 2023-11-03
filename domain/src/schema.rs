// @generated automatically by Diesel CLI.
/*
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
        created_at -> TIMESTAMPTZ,
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
        created_at -> TIMESTAMPTZ,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        start_time -> TIMESTAMPTZ,
        end_time -> TIMESTAMPTZ,
        lat -> Float8,
        lon -> Float8,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        content -> Varchar,
        attachment_id -> Nullable<Int4>,
        reply_to -> Nullable<Int4>,
        from_id -> Int4,
        to_id -> Int4,
        created_at -> TIMESTAMPTZ,
    }
}

diesel::table! {
    post_depicted_people (post_id, user_id) {
        post_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    post_downloads (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
    }
}

diesel::table! {
    post_likes (user_id, post_id) {
        user_id -> Int4,
        post_id -> Int4,
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
        created_at -> TIMESTAMPTZ,
    }
}

diesel::table! {
    user_blocked (user_id, blocked_id) {
        user_id -> Int4,
        blocked_id -> Int4,
    }
}

diesel::table! {
    user_follows (user_id, following_id) {
        user_id -> Int4,
        following_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        nickname -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        created_at -> TIMESTAMPTZ,
        twitter_username -> Nullable<Varchar>,
        instagram_username -> Nullable<Varchar>,
        tiktok_username -> Nullable<Varchar>,
        onlyfans_username -> Nullable<Varchar>,
        snapchat_username -> Nullable<Varchar>,
        youtube_username -> Nullable<Varchar>,
        myanimelist_username -> Nullable<Varchar>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(messages -> attachments (attachment_id));
diesel::joinable!(post_depicted_people -> posts (post_id));
diesel::joinable!(post_depicted_people -> users (user_id));
diesel::joinable!(post_downloads -> posts (post_id));
diesel::joinable!(post_downloads -> users (user_id));
diesel::joinable!(post_likes -> posts (post_id));
diesel::joinable!(post_likes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    comments,
    events,
    messages,
    post_depicted_people,
    post_downloads,
    post_likes,
    posts,
    user_blocked,
    user_follows,
    users,
);
*/
