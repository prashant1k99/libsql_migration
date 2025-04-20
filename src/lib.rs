use std::path::{Path, PathBuf};

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
        Err(LibsqlMigratorError::CustomError(String::from(
            "Invalid migration file",
        )))
    }
}

pub async fn migrate(
    conn: Connection,
    migrations_folder: PathBuf,
) -> Result<(), errors::LibsqlMigratorError> {
    validate_migration_folder(&migrations_folder)?;

    create_migration_table(&conn).await?;

    // get all the .sql file
    Ok(())
}
