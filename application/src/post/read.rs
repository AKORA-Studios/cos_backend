// application/src/post/read.rs

use std::time::{Duration, SystemTime};

use diesel::prelude::*;
use domain::{
    models::{FullPost, JoinedPostWithUser, POST_WITH_USER_COLUMNS},
    schema::posts::all_columns,
};

use rocket::response::status::NotFound;

use crate::util::map_diesel_result;

// !TODO Also return users if they already liked a post or not

pub fn view_post(db_conn: &mut PgConnection, post_id: i32) -> Result<FullPost, NotFound<String>> {
    use domain::schema::posts::dsl::*;
    use domain::schema::users;

    let result = posts
        .find(post_id)
        .inner_join(users::table.on(users::id.eq(user_id)))
        .select(POST_WITH_USER_COLUMNS)
        .first::<JoinedPostWithUser>(db_conn);

    map_diesel_result(match result {
        Ok(found_post) => {
            let info_result = get_post_info(&found_post, db_conn);

            match info_result {
                Ok((downloads, likes, depicted_people)) => {
                    Ok(found_post.convert(downloads, likes, depicted_people))
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    })
}

pub fn list_recent_posts(db_conn: &mut PgConnection, limit: usize) -> Vec<FullPost> {
    use domain::schema::posts::dsl::*;
    use domain::schema::users;

    let result = posts
        .order(created_at.desc())
        .limit(limit as i64)
        .inner_join(users::table.on(users::id.eq(user_id)))
        .select(POST_WITH_USER_COLUMNS)
        .load::<JoinedPostWithUser>(db_conn);

    match result {
        Ok(post_list) => post_list
            .iter()
            .map(|post: &JoinedPostWithUser| {
                let (downloads, likes, depicted_people) = get_post_info(post, db_conn).unwrap();

                post.convert(downloads, likes, depicted_people)
            })
            .collect(),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_today_posts(db_conn: &mut PgConnection, limit: usize) -> Vec<FullPost> {
    use domain::schema::posts::dsl::*;
    use domain::schema::users;

    let result = posts
        .select(all_columns)
        .filter(created_at.gt(SystemTime::now() - Duration::from_secs(60 * 60 * 24)))
        .limit(limit as i64)
        .inner_join(users::table.on(users::id.eq(user_id)))
        .select(POST_WITH_USER_COLUMNS)
        .load::<JoinedPostWithUser>(db_conn);

    match result {
        Ok(post_list) => post_list
            .iter()
            .map(|post: &JoinedPostWithUser| {
                let (downloads, likes, depicted_people) = get_post_info(post, db_conn).unwrap();

                post.convert(downloads, likes, depicted_people)
            })
            .collect(),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_user_posts(db_conn: &mut PgConnection, user_id: i32, limit: usize) -> Vec<FullPost> {
    use domain::schema::posts;
    use domain::schema::users;

    let result = posts::table
        .select(all_columns)
        .filter(posts::user_id.eq(user_id))
        .limit(limit as i64)
        .inner_join(users::table.on(users::id.eq(user_id)))
        .select(POST_WITH_USER_COLUMNS)
        .load::<JoinedPostWithUser>(db_conn);

    match result {
        Ok(post_list) => post_list
            .iter()
            .map(|post: &JoinedPostWithUser| {
                let (downloads, likes, depicted_people) = get_post_info(post, db_conn).unwrap();

                post.convert(downloads, likes, depicted_people)
            })
            .collect(),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

/*











*/

fn get_post_info(
    post: &JoinedPostWithUser,
    conn: &mut PgConnection,
) -> diesel::result::QueryResult<(i64, i64, Vec<i32>)> {
    use domain::schema::post_depicted_people;
    use domain::schema::post_downloads;
    use domain::schema::post_likes;

    let downloads: i64 = post_downloads::table
        .filter(post_downloads::post_id.eq(post.id))
        .count()
        .get_result(conn)?;

    let likes: i64 = post_likes::table
        .filter(post_likes::post_id.eq(post.id))
        .count()
        .get_result(conn)?;

    let depicted_people = post_depicted_people::table
        .filter(post_depicted_people::post_id.eq(post.id))
        .select(post_depicted_people::user_id)
        .load::<i32>(conn)?;

    Ok((downloads, likes, depicted_people))
}
