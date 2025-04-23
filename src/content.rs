//! Provides migration capabilities using SQL content provided directly as strings.
//!
//! This module is activated by the `content` feature. It allows applying migrations
//! where the SQL script is loaded or generated dynamically within the application,
//! rather than read from files. Each migration needs a unique ID.
//!
//! # Usage
//!
//! ```no_run
//! # #[cfg(feature = "content")]
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use libsql_migration::{content::migrate, errors::LibsqlContentMigratorError, util::MigrationResult};
//! use libsql::Builder;
//!
//! // Ensure the `content` feature is enabled and default features disabled in Cargo.toml
//! // [dependencies]
//! // libsql_migration = { version = "...", default-features = false, features = ["content"] }
//!
//! let db = Builder::new_local("my_database.db").build().await.unwrap();
//! let conn = db.connect().unwrap();
//!
//! let migration_id = "0001_create_users_content".to_string();
//! let migration_sql = "CREATE TABLE IF NOT EXISTS users_from_content (id INTEGER PRIMARY KEY);".to_string();
//!
//! match migrate(&conn, migration_id.clone(), migration_sql).await {
//!     Ok(MigrationResult::Executed) => println!("Content migration '{}' applied successfully.", migration_id),
//!     Ok(MigrationResult::AlreadyExecuted) => println!("Content migration '{}' was already applied.", migration_id),
//!     Err(e) => eprintln!("Content migration '{}' failed: {}", migration_id, e),
//! }
//! # Ok(())
//! # }
//! ```

use crate::errors::LibsqlContentMigratorError;
use crate::util::{MigrationResult, create_migration_table, execute_migration};
use libsql::Connection;

pub async fn migrate(
    conn: &Connection,
    migration_id: String,
    migration_script: String,
) -> Result<MigrationResult, LibsqlContentMigratorError> {
    if migration_id.is_empty() {
        return Err(LibsqlContentMigratorError::InvalidInput(
            "`migration_id` is empty".to_string(),
        ));
    }
    if migration_script.is_empty() {
        return Err(LibsqlContentMigratorError::InvalidInput(
            "`migration_script` is empty".to_string(),
        ));
    }

    create_migration_table(conn).await?;

    let res = execute_migration(conn, migration_id, migration_script).await?;

    Ok(res)
}
