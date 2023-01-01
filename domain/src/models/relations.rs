// domain/src/models.rs
use diesel::prelude::*;

use super::post::Post;
use super::user::User;
use crate::schema::{post_depicted_people, post_downloads, post_likes, user_followers};

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
#[diesel(table_name = user_followers)]
#[diesel(primary_key(user_id, follower_id))]
pub struct UserFollowers {
    pub user_id: i32,
    pub follower_id: i32,
}
