// application/src/user/read.rs

use domain::models::{DisplayUser, DISPLAY_USER_COLUMNS};
use sqlx::PgPool;

use crate::{map_sqlx_result, OpResult, OpSuc};

pub async fn view_user(conn: &PgPool, user_id: i32) -> OpResult<DisplayUser, String> {
    let sql = format!("SELECT {} FROM users WHERE id = ?", DISPLAY_USER_COLUMNS);

    let result = sqlx::query_as::<_, DisplayUser>(&sql)
        .bind(user_id)
        .fetch_one(conn)
        .await;

    map_sqlx_result(result.map(|v| OpSuc::Read(v)))
}
