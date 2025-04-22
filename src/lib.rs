//! `libsql_migration` provides a simple migration mechanism for libsql databases.
//!
//! It allows you to apply SQL migration files located in a specified directory
//! to your database, keeping track of which migrations have already been applied.
//!
//! # Usage
//!
//! **Note:** This crate relies on the `tokio` runtime for its asynchronous operations. Ensure `tokio` is included in your project dependencies and an appropriate runtime is available (e.g., using `#[tokio::main]`).
//!
//! ```no_run
//! use libsql_migration::{migrate, errors::LibsqlMigratorError};
//! use libsql::Builder;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), LibsqlMigratorError> {
//!     // Connect to your database
//!     let db = Builder::new_local("my_database.db").build().await.unwrap();
//!     let conn = db.connect().unwrap();
//!
//!     // Specify the path to your migration files
//!     let migrations_folder = PathBuf::from("./migrations");
//!
//!     // Run the migrations
//!     match migrate(&conn, migrations_folder).await {
//!         Ok(applied) => {
//!             if applied {
//!                 println!("Migrations applied successfully.");
//!             } else {
//!                 println!("No new migrations to apply.");
//!             }
//!         }
//!         Err(e) => {
//!             eprintln!("Migration failed: {}", e);
//!             return Err(e);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Migration Files
//!
//! Migration files should be plain SQL files (`.sql` extension) located within the
//! specified migrations directory. They are executed in lexicographical order based
//! on their filenames. It's recommended to prefix filenames with numbers or timestamps
//! to ensure correct ordering (e.g., `0001_initial.sql`, `0002_add_users_table.sql`).
//!
//! The migrator creates a `libsql_migrations` table in your database to track
//! applied migrations.
//!
//! [GitHub Repository](https://github.com/prashant1k99/libsql_migration)

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use errors::LibsqlMigratorError;
use libsql::Connection;

pub mod errors;

async fn create_migration_table(conn: &Connection) -> Result<(), LibsqlMigratorError> {
    let sql_query = include_str!("./base_migration_table.sql");

    conn.execute(sql_query, libsql::params![]).await?;
    Ok(())
}

fn validate_migration_folder(path: &Path) -> Result<(), LibsqlMigratorError> {
    if !path.exists() {
        return Err(LibsqlMigratorError::MigrationDirNotFound(
            path.to_path_buf(),
        ));
    };
    if path.is_dir() || (path.is_file() && path.extension().unwrap_or_default() == "sql") {
        Ok(())
    } else {
        Err(LibsqlMigratorError::InvalidMigrationPath(
            path.to_path_buf(),
        ))
    }
}

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
) -> Result<bool, errors::LibsqlMigratorError> {
    validate_migration_folder(&migrations_folder)?;

    create_migration_table(conn).await?;

    let files_to_run = check_dir_for_sql_files(migrations_folder.clone())
        .map_err(|e| LibsqlMigratorError::ErrorWhileGettingSQLFiles(e.to_string()))?;

    if files_to_run.is_empty() {
        return Ok(false);
    };

    let mut did_new_migration = false;

    for file in files_to_run {
        let file_id = file.strip_prefix(&migrations_folder).unwrap();

        let mut stmt = conn
            .prepare("SELECT status FROM libsql_migrations WHERE id = ?;")
            .await?;

        let mut rows = stmt.query([file_id.to_str()]).await?;

        if let Some(record) = rows.next().await? {
            let status_value = record.get_value(0)?;
            if let libsql::Value::Integer(1) = status_value {
                continue;
            }
        } else {
            did_new_migration = true;
            conn.execute(
                "INSERT INTO libsql_migrations (id) VALUES (?) ON CONFLICT(id) DO NOTHING",
                libsql::params![file_id.to_str()],
            )
            .await?;
        }

        let file_data = fs::read_to_string(&file).map_err(|_| {
            LibsqlMigratorError::ErrorWhileGettingSQLFiles(format!(
                "Unable to read {:?} file!",
                file_id.to_str()
            ))
        })?;

        conn.execute(&file_data, libsql::params!()).await?;

        conn.execute(
            "UPDATE libsql_migrations SET status = true, exec_time = CURRENT_TIMESTAMP WHERE id = ?",
            libsql::params![
                file_id.to_str()
            ],
        )
        .await?;
    }

    Ok(did_new_migration)
}

pub async fn run_migration(
    conn: &Connection,
    migration_id: String,
    migration_script: String,
) -> Result<(), errors::LibsqlMigratorError> {
    create_migration_table(conn).await?;

    let mut stmt = conn
        .prepare("SELECT status FROM libsql_migrations WHERE id = ?;")
        .await?;

    let mut rows = stmt.query([migration_id.clone()]).await?;

    if let Some(record) = rows.next().await? {
        let status_value = record.get_value(0)?;
        if let libsql::Value::Integer(1) = status_value {
            return Ok(());
        }
    }
    conn.execute(
        "INSERT INTO libsql_migrations (id) VALUES (?) ON CONFLICT(id) DO NOTHING",
        libsql::params![migration_id.clone()],
    )
    .await?;

    conn.execute(&migration_script, libsql::params!()).await?;

    conn.execute(
        "UPDATE libsql_migrations SET status = true, exec_time = CURRENT_TIMESTAMP WHERE id = ?",
        libsql::params![migration_id],
    )
    .await?;
    Ok(())
}

pub fn migrate_using_link(
    conn: &Connection,
    url: String,
) -> Result<(), errors::LibsqlMigratorError> {
    // Fetch the central endpoint and it should satisfy the URL serde condition
    Ok(())
}
