pub use shared::operation::*;

pub fn map_sqlx_result<T>(result: Result<T, sqlx::Error>) -> Result<T, OpErr<String>> {
    result.map_err(|e| match e {
        sqlx::Error::RowNotFound => OpErr::NotFound("".to_owned()),
        _ => {
            eprintln!("{e:?}");
            OpErr::internal_error()
        }
    })
}
