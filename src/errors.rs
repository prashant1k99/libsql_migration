use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    path::PathBuf,
};

use libsql::Error as LibsqlError;

#[derive(Debug)]
pub enum LibsqlMigratorBaseError {
    LibSqlError(LibsqlError),
    MigrationFailed(String),
}

impl Display for LibsqlMigratorBaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LibsqlMigratorBaseError::LibSqlError(e) => write!(f, "LibSqlError: {}", e),
            LibsqlMigratorBaseError::MigrationFailed(msg) => {
                write!(f, "LibsqlMigrationError: Migration failed | {}", msg)
            }
        }
    }
}

impl From<LibsqlError> for LibsqlMigratorBaseError {
    fn from(value: LibsqlError) -> Self {
        LibsqlMigratorBaseError::LibSqlError(value)
    }
}

impl Error for LibsqlMigratorBaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LibsqlMigratorBaseError::LibSqlError(e) => Some(e),
            _ => None,
        }
    }
}

// LibsqlDirMigratorError

#[derive(Debug)]
pub enum LibsqlDirMigratorError {
    BaseError(LibsqlMigratorBaseError),
    MigrationDirNotFound(PathBuf),
    InvalidMigrationPath(PathBuf),
    ErrorWhileGettingSQLFiles(String),
}

impl Display for LibsqlDirMigratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LibsqlDirMigratorError::BaseError(e) => write!(f, "{}", e),
            LibsqlDirMigratorError::MigrationDirNotFound(path) => {
                write!(
                    f,
                    "LibsqlDirMigratorError: {} path not found",
                    path.to_string_lossy()
                )
            }
            LibsqlDirMigratorError::InvalidMigrationPath(path) => {
                write!(
                    f,
                    "LibsqlDirMigratorError: {} unsupported migration path provided",
                    path.to_string_lossy()
                )
            }
            LibsqlDirMigratorError::ErrorWhileGettingSQLFiles(msg) => write!(
                f,
                "LibsqlDirMigratorError: Error occured while traversing migration folder | {}",
                msg
            ),
        }
    }
}

impl Error for LibsqlDirMigratorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LibsqlDirMigratorError::BaseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<LibsqlMigratorBaseError> for LibsqlDirMigratorError {
    fn from(value: LibsqlMigratorBaseError) -> Self {
        LibsqlDirMigratorError::BaseError(value)
    }
}

impl From<LibsqlError> for LibsqlDirMigratorError {
    fn from(value: LibsqlError) -> Self {
        LibsqlDirMigratorError::BaseError(LibsqlMigratorBaseError::LibSqlError(value))
    }
}

// LibsqlRemoteMigratorError

#[derive(Debug)]
pub enum LibsqlRemoteMigratorError {
    BaseError(LibsqlMigratorBaseError),
    MigrationUrlNotValid(String),
}

impl Display for LibsqlRemoteMigratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LibsqlRemoteMigratorError::BaseError(e) => write!(f, "{}", e),
            LibsqlRemoteMigratorError::MigrationUrlNotValid(string) => {
                write!(f, "LibsqlRemoteMigratorError: Invalid URL {}", string)
            }
        }
    }
}

impl Error for LibsqlRemoteMigratorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LibsqlRemoteMigratorError::BaseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<LibsqlMigratorBaseError> for LibsqlRemoteMigratorError {
    fn from(value: LibsqlMigratorBaseError) -> Self {
        LibsqlRemoteMigratorError::BaseError(value)
    }
}

impl From<LibsqlError> for LibsqlRemoteMigratorError {
    fn from(value: LibsqlError) -> Self {
        LibsqlRemoteMigratorError::BaseError(LibsqlMigratorBaseError::LibSqlError(value))
    }
}
