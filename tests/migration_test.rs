use libsql_migration::migrate;

#[cfg(test)]
mod migration_tests {
    mod base {
        use super::super::*;

        use std::path::PathBuf;
        use tempfile::tempdir;

        #[tokio::test]
        async fn establish_connection() -> Result<(), Box<dyn std::error::Error>> {
            let temp_dir = tempdir()?;

            let db_path = PathBuf::from(format!("{}/test.db", temp_dir.path().to_str().unwrap()));
            let db = libsql::Builder::new_local(db_path).build().await?;

            let conn = db.connect()?;

            migrate(conn.clone(), temp_dir.into_path()).await?;

            // Check for migrations_table
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name = 'libsql_migrations';")
                .await?;

            let mut rows = stmt.query(()).await?;

            assert!(rows.next().await?.is_some(), "Migrations table not found");

            Ok(())
        }

        #[tokio::test]
        async fn non_existent_migration_folder() -> Result<(), Box<dyn std::error::Error>> {
            let temp_dir = tempdir()?;

            let db_path = PathBuf::from(format!("{}/test.db", temp_dir.path().to_str().unwrap()));
            let db = libsql::Builder::new_local(db_path).build().await?;

            let conn = db.connect()?;

            let non_existent_migration_folder = PathBuf::from("./my_non_existent_migrations/");
            match migrate(conn, non_existent_migration_folder).await {
                Ok(_) => Err(Box::from(
                    "Should have thrown error for non-existent folder",
                )),
                Err(e) => {
                    println!("Error: {:#?}", e);
                    Ok(())
                }
            }
        }

        #[tokio::test]
        async fn invalid_file_as_migration() -> Result<(), Box<dyn std::error::Error>> {
            let temp_dir = tempdir()?;

            let db_path = PathBuf::from(format!("{}/test.db", temp_dir.path().to_str().unwrap()));
            let db = libsql::Builder::new_local(db_path).build().await?;

            let conn = db.connect()?;

            let migration_dir = temp_dir.path();
            let invalid_file_path = migration_dir.join("invalid_migration.txt");
            std::fs::write(&invalid_file_path, "this is not sql")?; // Create an invalid file

            match migrate(conn, invalid_file_path).await {
                Ok(_) => Err(Box::from(
                    "Should have thrown error for non-existent folder",
                )),
                Err(e) => {
                    println!("Error: {:#?}", e);
                    Ok(())
                }
            }
        }
    }

    // mod subtraction {
    //     // ... subtraction tests ...
    // }
}
