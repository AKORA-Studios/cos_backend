// application/src/user/read.rs

use domain::models::{DisplayUser, PatchedUser, DISPLAY_USER_COLUMNS};

use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{map_sqlx_result, OpErr, OpResult, OpSuc};

pub async fn modify_user(
    conn: &PgPool,
    user_id: i32,
    patch_data: PatchedUser,
) -> OpResult<DisplayUser, sqlx::Error> {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(r#"UPDATE "users" SET "#);

    //    let sep = query.separated(", ");

    if patch_data.nickname.is_some() {
        query
            .push("nickname")
            .push("=")
            .push_bind(patch_data.nickname);
    } else {
        return Err(OpErr::Any);
    }

    query.push(" WHERE id = ").push_bind(user_id);
    query.push(format!(" RETURNING {}", DISPLAY_USER_COLUMNS));

    let finished_query = query.build_query_as::<DisplayUser>();

    let result = finished_query.fetch_one(conn).await;

    map_sqlx_result(result.map(|v| OpSuc::Updated(v)))
}
