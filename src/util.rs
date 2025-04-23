use std::path::Path;

use crate::errors::{LibsqlDirMigratorError, LibsqlMigratorBaseError};
use libsql::Connection;

pub(crate) async fn create_migration_table(
    conn: &Connection,
) -> Result<(), LibsqlMigratorBaseError> {
    let sql_query = include_str!("./base_migration_table.sql");

    conn.execute(sql_query, libsql::params![]).await?;
    Ok(())
}

pub(crate) fn validate_migration_folder(path: &Path) -> Result<(), LibsqlDirMigratorError> {
    if !path.exists() {
        return Err(LibsqlDirMigratorError::MigrationDirNotFound(
            path.to_path_buf(),
        ));
    };
    if path.is_dir() || (path.is_file() && path.extension().unwrap_or_default() == "sql") {
        Ok(())
    } else {
        Err(LibsqlDirMigratorError::InvalidMigrationPath(
            path.to_path_buf(),
        ))
    }
}

pub enum MigrationResult {
    Executed,
    AlreadyExecuted,
}

pub(crate) async fn execute_migration(
    conn: &Connection,
    id: String,
    sql_script: String,
) -> Result<MigrationResult, LibsqlMigratorBaseError> {
    let mut stmt = conn
        .prepare("SELECT status FROM libsql_migrations WHERE id = ?;")
        .await?;

    let mut rows = stmt.query([id.clone()]).await?;

    if let Some(record) = rows.next().await? {
        let status_value = record.get_value(0)?;
        if let libsql::Value::Integer(1) = status_value {
            return Ok(MigrationResult::AlreadyExecuted);
        }
    } else {
        conn.execute(
            "INSERT INTO libsql_migrations (id) VALUES (?) ON CONFLICT(id) DO NOTHING",
            libsql::params![id.clone()],
        )
        .await?;
    }

    conn.execute(&sql_script, libsql::params!()).await?;

    conn.execute(
        "UPDATE libsql_migrations SET status = true, exec_time = CURRENT_TIMESTAMP WHERE id = ?",
        libsql::params![id],
    )
    .await?;

    Ok(MigrationResult::Executed)
}
