// domain/src/models.rs
use diesel::prelude::*;

use super::post::Post;
use super::user::User;
use crate::schema::{post_depicted_people, post_downloads, post_likes, user_blocked, user_follows};

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[diesel(belongs_to(Post, foreign_key = post_id))]
#[diesel(table_name = post_depicted_people)]
#[diesel(primary_key(post_id, user_id))]
pub struct PostDepictedPeople {
    pub post_id: i32,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[diesel(belongs_to(Post, foreign_key = post_id))]
#[diesel(table_name = post_downloads)]
#[diesel(primary_key(post_id, user_id))]
pub struct PostDownloads {
    pub post_id: i32,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[diesel(belongs_to(Post, foreign_key = post_id))]
#[diesel(table_name = post_likes)]
#[diesel(primary_key(post_id, user_id))]
pub struct PostLikes {
    pub post_id: i32,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = user_follows)]
#[diesel(primary_key(user_id, following_id))]
pub struct UserFollows {
    pub user_id: i32,
    pub following_id: i32,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = user_blocked)]
#[diesel(primary_key(user_id, blocked_id))]
pub struct UserBlocked {
    pub user_id: i32,
    pub blocked_id: i32,
}
