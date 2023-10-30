// infrastructure/src/lib.rs
/*
use rocket_sync_db_pools::{database, diesel};

#[database("cos")]
pub struct DbConn(diesel::PgConnection);

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(
        __r: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<Self, ()> {
        rocket_sync_db_pools::Connection::<DbConn, diesel::PgConnection>::from_request(__r)
            .await
            .map(Self)
    }
}
 */

pub async fn run_migrations() {
    sqlx::migrate!("migrations")
}
