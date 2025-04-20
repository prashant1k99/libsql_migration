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

    for file in files_to_run {
        let file_id = file.strip_prefix(&migrations_folder).unwrap();

        conn.execute(
            "INSERT INTO libsql_migrations (id) VALUES (?) ON CONFLICT(id) DO NOTHING",
            libsql::params![file_id.to_str()],
        )
        .await?;
    }

    Ok(true)
}
