// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Varchar,
        user_id -> Int4,
        post_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        during -> Tsrange,
        lat -> Float8,
        lon -> Float8,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        content -> Varchar,
        from_id -> Int4,
        to_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    post_people (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        description -> Varchar,
        user_id -> Int4,
        downloads -> Int4,
        likes -> Int4,
        tags -> Array<Nullable<Text>>,
        photographer_id -> Nullable<Int4>,
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
diesel::joinable!(post_people -> posts (post_id));
diesel::joinable!(post_people -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    events,
    messages,
    post_people,
    posts,
    users,
);
