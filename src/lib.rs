use std::{path::PathBuf, sync::OnceLock};

use errors::LibsqlMigratorError;
use libsql::Connection;

pub mod errors;

static MIGRATIONS_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn setup(migrationPath: PathBuf) -> Result<(), errors::LibsqlMigratorError> {
    MIGRATIONS_DIR
        .set(migrationPath)
        .map_err(|_| LibsqlMigratorError::MigrationFolderAlreadyInit);
    Ok(())
}

pub fn migrate(conn: Connection) -> Result<(), errors::LibsqlMigratorError> {
    // Create migration table for handling migrations
    // get all the .sql file
    Ok(())
}
