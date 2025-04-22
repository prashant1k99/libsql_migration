use std::path::Path;

use crate::errors::LibsqlMigratorError;
use libsql::Connection;

pub(crate) async fn create_migration_table(conn: &Connection) -> Result<(), LibsqlMigratorError> {
    let sql_query = include_str!("./base_migration_table.sql");

    conn.execute(sql_query, libsql::params![]).await?;
    Ok(())
}

pub(crate) fn validate_migration_folder(path: &Path) -> Result<(), LibsqlMigratorError> {
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
