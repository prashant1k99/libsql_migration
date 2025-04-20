use libsql_migration::migrate;
use std::path::PathBuf;
use tempfile::tempdir;

#[cfg(test)]
mod migration_tests {
    use tempfile::TempDir;

    use super::*;

    async fn setup_test_db() -> Result<(libsql::Connection, TempDir), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let db = libsql::Builder::new_local(db_path).build().await?;
        let conn = db.connect()?;
        // Return both connection and temp_dir to keep the directory alive
        Ok((conn, temp_dir))
    }

    mod base {
        use libsql_migration::errors::LibsqlMigratorError;

        use crate::migration_tests::setup_test_db;

        use super::super::*;

        #[tokio::test]
        async fn establish_connection() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, temp_dir) = setup_test_db().await?;

            migrate(&conn, temp_dir.into_path()).await?;

            // Check for migrations_table
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name = ?;")
                .await?;

            let mut rows = stmt.query(["libsql_migrations"]).await?;

            assert!(rows.next().await?.is_some(), "Migrations table not found");

            Ok(())
        }

        #[tokio::test]
        async fn non_existent_migration_folder() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _) = setup_test_db().await?;

            let non_existent_migration_folder = PathBuf::from("./my_non_existent_migrations/");
            match migrate(&conn, non_existent_migration_folder).await {
                Ok(_) => Err(Box::from(
                    "Should have thrown error for non-existent folder",
                )),
                Err(e) => {
                    assert!(
                        matches!(e, LibsqlMigratorError::MigrationDirNotFound(_)),
                        "Expected MigrationDirNotFound, got {:?}",
                        e
                    );
                    println!("Error: {:#?}", e);
                    Ok(())
                }
            }
        }

        #[tokio::test]
        async fn invalid_file_as_migration() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, temp_dir) = setup_test_db().await?;

            let migration_dir = temp_dir.path();
            let invalid_file_path = migration_dir.join("invalid_migration.txt");
            std::fs::write(&invalid_file_path, "this is not sql")?; // Create an invalid file

            match migrate(&conn, invalid_file_path).await {
                Ok(_) => Err(Box::from(
                    "Should have thrown error for non-existent folder",
                )),
                Err(e) => {
                    assert!(
                        matches!(e, LibsqlMigratorError::InvalidMigrationPath(_)),
                        "Expected InvalidMigrationPath, got {:?}",
                        e
                    );
                    Ok(())
                }
            }
        }
    }

    mod migration {
        use std::fs;

        use super::super::*;
        use crate::migration_tests::setup_test_db;

        #[tokio::test]
        async fn test_for_file_counts() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, temp_dir) = setup_test_db().await?;

            let migration_dir = temp_dir.path().join("migrations");
            fs::create_dir_all(&migration_dir.join("test"))?;

            std::fs::write(migration_dir.join("0001_test1.sql"), "this is not sql")?; // Create an invalid file
            std::fs::write(migration_dir.join("test/0001_test0.sql"), "this is not sql")?; // Create an invalid file
            std::fs::write(migration_dir.join("0002_est2.sql"), "this is not sql")?; // Create an invalid file
            std::fs::write(migration_dir.join("0003_test3.sql"), "this is not sql")?; // Create an invalid file
            std::fs::write(migration_dir.join("0004_test4.sql"), "this is not sql")?; // Create an invalid file

            migrate(&conn, temp_dir.into_path()).await?;

            Ok(())
        }
    }

    // Test migrations
    // 1. Test file numeric order
    // 2. Test if the migration is successfull, then does it adds the records in migration_table
    // 3. When migration is started again it should not run the migration on already executed
    //    queries

    // mod subtraction {
    //     // ... subtraction tests ...
    // }
}
