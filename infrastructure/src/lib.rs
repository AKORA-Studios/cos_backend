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
    // migrations dir concat!(env!("CARGO_MANIFEST_DIR"), "/migrations")
    let mut migrations = Vec::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        if !fs::metadata(entry.path())?.is_file() {
            // not a file; ignore
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        let parts = file_name.splitn(2, '_').collect::<Vec<_>>();

        if parts.len() != 2 || !parts[1].ends_with(".sql") {
            // not of the format: <VERSION>_<DESCRIPTION>.sql; ignore
            continue;
        }

        let version: i64 = parts[0].parse()?;

        let migration_type = MigrationType::from_filename(parts[1]);
        // remove the `.sql` and replace `_` with ` `
        let description = parts[1]
            .trim_end_matches(migration_type.suffix())
            .replace('_', " ")
            .to_owned();

        let sql = fs::read_to_string(&entry.path())?;

        let checksum = Vec::from(Sha384::digest(sql.as_bytes()).as_slice());

        // canonicalize the path so we can pass it to `include_str!()`
        let path = entry.path().canonicalize()?;
        let path = path
            .to_str()
            .ok_or_else(|| {
                format!(
                    "migration path cannot be represented as a string: {:?}",
                    path
                )
            })?
            .to_owned();

        migrations.push(QuotedMigration {
            version,
            description,
            migration_type: QuotedMigrationType(migration_type),
            path,
            checksum,
        })
    }

    // ensure that we are sorted by `VERSION ASC`
    migrations.sort_by_key(|m| m.version);

    #[cfg(any(sqlx_macros_unstable, procmacro2_semver_exempt))]
    {
        let path = path.canonicalize()?;
        let path = path.to_str().ok_or_else(|| {
            format!(
                "migration directory path cannot be represented as a string: {:?}",
                path
            )
        })?;

        proc_macro::tracked_path::path(path);
    }

    Ok(quote! {
        ::sqlx::migrate::Migrator {
            migrations: ::std::borrow::Cow::Borrowed(&[
                #(#migrations),*
            ]),
            ignore_missing: false,
            locking: true,
        }
    })
}
