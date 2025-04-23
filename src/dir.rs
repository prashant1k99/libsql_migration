//! Provides migration capabilities using SQL files from a local directory.
//!
//! This module is activated by the `dir` feature (enabled by default).
//! It finds `.sql` files in a specified directory, sorts them lexicographically,
//! and applies them sequentially if they haven't been applied before.
//!
//! # Usage
//!
//! ```no_run
//! # #[cfg(feature = "dir")]
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use libsql_migration::{dir::migrate, errors::LibsqlDirMigratorError};
//! use libsql::Builder;
//! use std::path::PathBuf;
//!
//! // Ensure the `dir` feature is enabled in Cargo.toml
//! // [dependencies]
//! // libsql_migration = { version = "...", features = ["dir"] } # Or keep default features
//!
//! let db = Builder::new_local("my_database.db").build().await.unwrap();
//! let conn = db.connect().unwrap();
//!
//! // Specify the path to your migration files (e.g., ./migrations)
//! let migrations_folder = PathBuf::from("./migrations");
//! // Ensure this directory exists and contains files like:
//! // - 0001_create_users.sql
//! // - 0002_add_email_to_users.sql
//!
//! match migrate(&conn, migrations_folder).await {
//!     Ok(applied) => {
//!         if applied {
//!             println!("Directory migrations applied successfully.");
//!         } else {
//!             println!("No new directory migrations to apply.");
//!         }
//!     }
//!     Err(e) => eprintln!("Directory migration failed: {}", e),
//! }
//! # Ok(())
//! # }
//! ```

use crate::errors::LibsqlDirMigratorError;
use crate::util::{
    MigrationResult, create_migration_table, execute_migration, validate_migration_folder,
};
use libsql::Connection;
use std::{fs, io, path::PathBuf};

fn check_dir_for_sql_files(root_path: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut file_paths: Vec<PathBuf> = vec![];
    let mut path_to_visit: Vec<PathBuf> = vec![root_path];

    while let Some(current_path) = path_to_visit.pop() {
        if current_path.is_dir() {
            for entry in fs::read_dir(current_path)? {
                let entry_path = entry?.path();
                path_to_visit.push(entry_path);
            }
        } else if current_path.is_file() && current_path.extension().is_some_and(|ext| ext == "sql")
        {
            file_paths.push(current_path);
        }
    }

    file_paths.sort_by(|a, b| {
        let a_name = a.file_name().unwrap_or_default();
        let b_name = b.file_name().unwrap_or_default();
        a_name.cmp(b_name)
    });

    Ok(file_paths)
}

pub async fn migrate(
    conn: &Connection,
    migrations_folder: PathBuf,
) -> Result<bool, LibsqlDirMigratorError> {
    validate_migration_folder(&migrations_folder)?;

    create_migration_table(conn).await?;

    let files_to_run = check_dir_for_sql_files(migrations_folder.clone())
        .map_err(|e| LibsqlDirMigratorError::ErrorWhileGettingSQLFiles(e.to_string()))?;

    if files_to_run.is_empty() {
        return Ok(false);
    };

    let mut did_new_migration = false;

    for file in files_to_run {
        let file_id = file.strip_prefix(&migrations_folder).unwrap();

        let file_data = fs::read_to_string(&file).map_err(|_| {
            LibsqlDirMigratorError::ErrorWhileGettingSQLFiles(format!(
                "Unable to read {:?} file!",
                file_id.to_str()
            ))
        })?;

        if let MigrationResult::Executed =
            execute_migration(conn, file_id.to_str().unwrap().to_string(), file_data).await?
        {
            did_new_migration = true
        }
    }

    Ok(did_new_migration)
}
