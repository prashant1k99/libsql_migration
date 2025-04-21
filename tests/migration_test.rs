use libsql_migration::migrate;
use std::path::PathBuf;
use tempfile::tempdir;

#[cfg(test)]
mod migration_tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    async fn setup_test_db()
    -> Result<(libsql::Connection, TempDir, PathBuf), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let db = libsql::Builder::new_local(db_path).build().await?;
        let conn = db.connect()?;

        let migration_dir = temp_dir.path().join("migrations");
        fs::create_dir_all(migration_dir.join("test"))?;

        std::fs::write(
            migration_dir.join("0001_test1.sql"),
            "CREATE TABLE test1 (
  id INTEGER PRIMARY KEY autoincrement
);",
        )?; // Create an invalid file
        std::fs::write(
            migration_dir.join("test/0001_test0.sql"),
            "CREATE TABLE test2 (
  id INTEGER PRIMARY KEY autoincrement
);",
        )?; // Create an invalid file
        std::fs::write(
            migration_dir.join("0002_est2.sql"),
            "ALTER TABLE test1
ADD Email TEXT;",
        )?; // Create an invalid file
        std::fs::write(
            migration_dir.join("0003_test3.sql"),
            "ALTER TABLE test1
ADD status BOOLEAN DEFAULT true;",
        )?; // Create an invalid file
        std::fs::write(
            migration_dir.join("0004_test4.sql"),
            "ALTER TABLE test2
ADD status boolean default true;",
        )?; // Create an invalid file

        // Return both connection and temp_dir to keep the directory alive
        Ok((conn, temp_dir, migration_dir))
    }

    mod base {
        use libsql_migration::errors::LibsqlMigratorError;

        use crate::migration_tests::setup_test_db;

        use super::super::*;

        #[tokio::test]
        async fn establish_connection() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, temp_dir, _) = setup_test_db().await?;

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
            let (conn, _, _) = setup_test_db().await?;

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
            let (conn, temp_dir, _) = setup_test_db().await?;

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

        use super::super::*;
        use crate::migration_tests::setup_test_db;

        #[tokio::test]
        async fn test_for_file_counts() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir, migration_dir) = setup_test_db().await?;

            migrate(&conn, migration_dir.to_path_buf()).await?;

            // Check for migrations_table
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table';")
                .await?;

            let mut rows = stmt.query(()).await?;

            let mut all_table_records: Vec<libsql::Value> = vec![];

            while let Some(row) = rows.next().await? {
                all_table_records.push(row.get_value(0)?);
            }

            assert!(
                all_table_records.contains(&libsql::Value::Text(String::from("test1"))),
                "Failed to create test1 migrations"
            );

            assert!(
                all_table_records.contains(&libsql::Value::Text(String::from("test2"))),
                "Failed to create test2 migrations"
            );

            let mut rows = conn
                .query("PRAGMA table_info('test1');", libsql::params![])
                .await?;

            let mut column_info: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();

            while let Some(row) = rows.next().await? {
                let name = row.get::<String>(1)?;
                let type_name = row.get::<String>(2)?;
                column_info.insert(name, type_name);
            }

            assert_eq!(
                column_info.get_key_value("id"),
                Some((&String::from("id"), &String::from("INTEGER")))
            );

            assert_eq!(
                column_info.get_key_value("id"),
                Some((&String::from("id"), &String::from("INTEGER")))
            );

            assert_eq!(
                column_info.get_key_value("id"),
                Some((&String::from("id"), &String::from("INTEGER")))
            );

            Ok(())
        }
    }

    // Test migrations
    // 1. Test file numeric order
    // 2. Test if the migration is successfull, then does it adds the records in migration_table
    // 3. When migration is started again it should not run the migration on already executed
    //    queries
}
