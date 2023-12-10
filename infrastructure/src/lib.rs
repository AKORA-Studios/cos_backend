// infrastructure/src/lib.rs
use std::{
    borrow::Cow,
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use futures_core::future::BoxFuture;
use sqlx::{
    error::BoxDynError,
    migrate::{Migrate, MigrateError, Migration, MigrationSource, MigrationType, Migrator},
    Acquire,
};

#[derive(Debug)]
pub struct CustomSource(pub PathBuf);
impl CustomSource {
    fn new(path: &str) -> Self {
        CustomSource(path.into())
    }
}
impl Deref for CustomSource {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CustomSource {
    // Format: <YEAR>_<MONTH>_<DAY>_<VERSION>_<DESCRIPTION>.sql;
    fn resolve_child(path: &Path, parent: &str) -> Result<Vec<Migration>, BoxDynError> {
        let mut s = fs::read_dir(path.canonicalize()?)?;
        let mut migrations = Vec::new();

        while let Some(entry) = s.next() {
            let entry = entry?;

            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            // Resolve all symlinks
            let entry_path = entry.path().canonicalize()?;

            // Check for dir with possible children
            if std::fs::metadata(&entry_path)?.is_dir() {
                let new_parent = format!("{}{}_", parent, file_name);
                let mut children = CustomSource::resolve_child(&entry.path(), &new_parent)?;
                migrations.append(&mut children);
                continue;
            }

            let full_name = format!("{parent}{file_name}").replace("-", "_");
            let parts = full_name.splitn(5, '_').collect::<Vec<_>>();

            if parts.len() != 5 || !parts[4].ends_with(".sql") {
                // not correct format
                continue;
            }

            let (year, month, day, version, description) = if let [y, m, d, v, desc] = &parts[0..5]
            {
                (*y, *m, *d, *v, *desc)
            } else {
                // This should never happen
                continue;
            };

            let version: i64 = format!("{year}{month}{day}{version}").parse()?;

            let migration_type = MigrationType::from_filename(description);
            // remove the `.sql` and replace `_` with ` `
            let description = description
                .trim_end_matches(migration_type.suffix())
                .replace('_', " ")
                .to_owned();

            let sql = fs::read_to_string(&entry.path())?;

            migrations.push(Migration::new(
                version,
                Cow::Owned(description),
                migration_type,
                Cow::Owned(sql),
            ));
        }

        // Enough to do it once at the end in the `resolve function`
        // ensure that we are sorted by `VERSION ASC`
        // migrations.sort_by_key(|m| m.version);

        Ok(migrations)
    }
}

impl MigrationSource<'static> for CustomSource {
    fn resolve(self) -> BoxFuture<'static, Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async move {
            let entry_path = self.canonicalize()?;
            let mut migrations = Vec::new();

            let mut children = CustomSource::resolve_child(&entry_path, "")?;
            migrations.append(&mut children);

            // ensure that we are sorted by `VERSION ASC`
            migrations.sort_by_key(|m| m.version);

            Ok(migrations)
        })
    }
}

pub async fn run_migrations<'a, A>(pool: A) -> Result<(), MigrateError>
where
    A: Acquire<'a>,
    <A::Connection as Deref>::Target: Migrate,
{
    let migration_path = std::env::current_dir()
        .expect("Unable to get CWD")
        .join("infrastructure/migrations");
    let migration_path = migration_path.to_str().unwrap();

    let path = CustomSource::new(migration_path);

    let migrator = Migrator::new(path).await?;

    // Revert all migrations
    // migrator.undo(pool, 0).await

    // Run all migrations
    migrator.run(pool).await
}

/*
/// For testing purposes only
pub async fn build_migrator() -> Result<Migrator, MigrateError> {
    let migration_path = std::env::current_dir()
        .expect("Unable to get CWD")
        .join("migrations");
    let migration_path = migration_path.to_str().unwrap();

    let path = CustomSource::new(migration_path);

    Migrator::new(path).await
}
*/
