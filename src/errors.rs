use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    path::PathBuf,
};

use libsql::Error as LibsqlError;

#[derive(Debug)]
pub enum LibsqlMigratorError {
    LibSqlError(LibsqlError),
    MigrationFailed(String),
    MigrationDirNotFound(PathBuf),
    MigrationFolderAlreadyInit,
    CustomError(String),
}

impl Display for LibsqlMigratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LibsqlMigratorError::LibSqlError(e) => write!(f, "LibSqlError: {}", e),
            LibsqlMigratorError::MigrationFailed(msg) => {
                write!(f, "LibsqlMigrationError: Migration failed | {}", msg)
            }
            LibsqlMigratorError::MigrationFolderAlreadyInit => {
                write!(f, "LibsqlMigratorError: Migration Dir already initialized")
            }
            LibsqlMigratorError::MigrationDirNotFound(path) => {
                write!(
                    f,
                    "LibsqlMigratorError: {} path not found",
                    path.to_string_lossy()
                )
            }
            LibsqlMigratorError::CustomError(e) => write!(f, "LibsqlMigrationError: {}", e),
        }
    }
}

impl Error for LibsqlMigratorError {}

impl From<LibsqlError> for LibsqlMigratorError {
    fn from(value: LibsqlError) -> Self {
        LibsqlMigratorError::LibSqlError(value)
    }
}
