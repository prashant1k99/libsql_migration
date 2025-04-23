//! Provides migration capabilities using SQL files fetched from a remote source.
//!
//! This module is activated by the `remote` feature. It expects a URL pointing to
//! a JSON endpoint that returns a list of migration objects, each containing an `id`
//! and a `url` pointing to the actual SQL script. Migrations are sorted by `id`
//! and applied sequentially.
//!
//! # Usage
//!
//! ```no_run
//! # #[cfg(feature = "remote")]
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use libsql_migration::{remote::migrate, errors::LibsqlRemoteMigratorError};
//! use libsql::Builder;
//!
//! // Ensure the `remote` feature is enabled and default features disabled in Cargo.toml
//! // [dependencies]
//! // libsql_migration = { version = "...", default-features = false, features = ["remote"] }
//! // reqwest = { version = "...", features = ["json"] } # Required by remote feature
//! // serde = { version = "...", features = ["derive"] } # Required by remote feature
//!
//! let db = Builder::new_local("my_database.db").build().await.unwrap();
//! let conn = db.connect().unwrap();
//!
//! // URL pointing to a JSON array like:
//! // [
//! //   { "id": "0001_remote_init", "url": "http://example.com/migrations/0001.sql" },
//! //   { "id": "0002_remote_users", "url": "http://example.com/migrations/0002.sql" }
//! // ]
//! let remote_migrations_url = "http://example.com/migrations.json".to_string();
//!
//! match migrate(&conn, remote_migrations_url).await {
//!     Ok(applied) => {
//!         if applied {
//!             println!("Remote migrations applied successfully.");
//!         } else {
//!             println!("No new remote migrations to apply.");
//!         }
//!     }
//!     Err(e) => eprintln!("Remote migration failed: {}", e),
//! }
//! # Ok(())
//! # }
//! ```

use crate::errors::LibsqlRemoteMigratorError;
use crate::util::{MigrationResult, create_migration_table, execute_migration};
use libsql::Connection;

#[derive(serde::Deserialize, Debug)]
struct RemoteMigrationFileSchema {
    id: String,
    url: String,
}

async fn make_request(
    url: String,
) -> Result<Vec<RemoteMigrationFileSchema>, LibsqlRemoteMigratorError> {
    let mut files = reqwest::get(url)
        .await?
        .json::<Vec<RemoteMigrationFileSchema>>()
        .await?;

    files.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(files)
}

async fn get_file_content(url: String) -> Result<String, LibsqlRemoteMigratorError> {
    let content = reqwest::get(url).await?.text().await?;

    Ok(content)
}

pub async fn migrate(conn: &Connection, url: String) -> Result<bool, LibsqlRemoteMigratorError> {
    create_migration_table(conn).await?;

    let all_files = make_request(url).await?;

    let mut did_new_migration = false;
    for file in all_files {
        let content = get_file_content(file.url).await?;
        if let MigrationResult::Executed = execute_migration(conn, file.id, content).await? {
            did_new_migration = true
        }
    }

    Ok(did_new_migration)
}
