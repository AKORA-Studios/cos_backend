// infrastructure/src/lib.rs
use std::{ops::Deref, path::Path};

use sqlx::{
    migrate::{Migrate, MigrateError, Migrator},
    Acquire,
};

pub async fn run_migrations<'a, A>(pool: A) -> Result<(), MigrateError>
where
    A: Acquire<'a>,
    <A::Connection as Deref>::Target: Migrate,
{
    let migration_path_str = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
    let path = Path::new(migration_path_str);

    let mirator = Migrator::new(path).await?;

    mirator.run(pool).await
}
