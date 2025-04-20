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
    InvalidMigrationPath(PathBuf),
}

impl Display for LibsqlMigratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LibsqlMigratorError::LibSqlError(e) => write!(f, "LibSqlError: {}", e),
            LibsqlMigratorError::MigrationFailed(msg) => {
                write!(f, "LibsqlMigrationError: Migration failed | {}", msg)
            }
            LibsqlMigratorError::MigrationDirNotFound(path) => {
                write!(
                    f,
                    "LibsqlMigratorError: {} path not found",
                    path.to_string_lossy()
                )
            }
            LibsqlMigratorError::InvalidMigrationPath(path) => {
                write!(
                    f,
                    "LibsqlMigratorError: {} unsupported migration path provided",
                    path.to_string_lossy()
                )
            }
        }
    }
}

impl Error for LibsqlMigratorError {}

impl From<LibsqlError> for LibsqlMigratorError {
    fn from(value: LibsqlError) -> Self {
        LibsqlMigratorError::LibSqlError(value)
    }
}
