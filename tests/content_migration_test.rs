use libsql_migration::content::migrate;
use tempfile::tempdir;

#[cfg(test)]
#[cfg(feature = "content")]
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
        use libsql_migration::errors::LibsqlContentMigratorError;

        use crate::migration_tests::setup_test_db;

        use super::super::*;

        #[tokio::test]
        async fn establish_connection() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(
                &conn,
                "001".to_string(),
                "CREATE TABLE test1 (
  id INTEGER PRIMARY KEY autoincrement
);"
                .to_string(),
            )
            .await?;

            // Check for migrations_table
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name = ?;")
                .await?;

            let mut rows = stmt.query(["libsql_migrations"]).await?;

            assert!(rows.next().await?.is_some(), "Migrations table not found");

            Ok(())
        }

        #[tokio::test]
        async fn invalid_migration_id() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            match migrate(
                &conn,
                "".to_string(),
                "CREATE TABLE test1 (
  id INTEGER PRIMARY KEY autoincrement
);"
                .to_string(),
            )
            .await
            {
                Ok(_) => Err(Box::from("Should have thrown error for empty migration_id")),
                Err(e) => {
                    assert!(
                        matches!(e, LibsqlContentMigratorError::InvalidInput(_)),
                        "Expected LibsqlContentMigratorError, got {:?}",
                        e
                    );
                    Ok(())
                }
            }
        }

        #[tokio::test]
        async fn invalid_migration_script() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            match migrate(&conn, "001".to_string(), "".to_string()).await {
                Ok(_) => Err(Box::from(
                    "Should have thrown error for empty migration_script",
                )),
                Err(e) => {
                    assert!(
                        matches!(e, LibsqlContentMigratorError::InvalidInput(_)),
                        "Expected LibsqlContentMigratorError, got {:?}",
                        e
                    );
                    Ok(())
                }
            }
        }
    }

    mod migration {
        use libsql_migration::util::MigrationResult;

        use super::super::*;
        use crate::migration_tests::setup_test_db;

        #[tokio::test]
        async fn check_for_script_execution() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(
                &conn,
                "001".to_string(),
                "CREATE TABLE test1 (
  id INTEGER PRIMARY KEY autoincrement
);"
                .to_string(),
            )
            .await?;

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

            Ok(())
        }

        #[tokio::test]
        async fn test_for_same_migration_id() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(
                &conn,
                "001".to_string(),
                "CREATE TABLE test1 (
  id INTEGER PRIMARY KEY autoincrement
);"
                .to_string(),
            )
            .await?;

            let res = migrate(
                &conn,
                "001".to_string(),
                "ALTER TABLE test1
ADD Email TEXT;"
                    .to_string(),
            )
            .await?;
            assert_eq!(res, MigrationResult::AlreadyExecuted);

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

            assert_eq!(column_info.get_key_value("Email"), None);

            Ok(())
        }

        #[tokio::test]
        async fn testing_different_migration_id() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(
                &conn,
                "001".to_string(),
                "CREATE TABLE test1 (
  id INTEGER PRIMARY KEY autoincrement
);"
                .to_string(),
            )
            .await?;

            migrate(
                &conn,
                "002".to_string(),
                "ALTER TABLE test1
ADD Email TEXT;"
                    .to_string(),
            )
            .await?;

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
                column_info.get_key_value("Email"),
                Some((&String::from("Email"), &String::from("TEXT")))
            );

            Ok(())
        }
    }
}
