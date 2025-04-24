use libsql_migration::remote::migrate;

#[cfg(test)]
mod migration_tests {
    use tempfile::tempdir;

    async fn setup_test_db()
    -> Result<(libsql::Connection, tempfile::TempDir), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let db = libsql::Builder::new_local(db_path).build().await?;
        let conn = db.connect()?;

        Ok((conn, temp_dir))
    }

    mod base {
        use libsql_migration::errors::LibsqlRemoteMigratorError;

        use super::super::*;
        use crate::migration_tests::setup_test_db;

        #[tokio::test]
        async fn establish_connection() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(&conn, String::from("https://raw.githubusercontent.com/prashant1k99/libsql_migration/refs/heads/main/tests/remote-sql/v1.json")).await?;

            // Check for migrations_table
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name = ?;")
                .await?;

            let mut rows = stmt.query(["libsql_migrations"]).await?;

            assert!(rows.next().await?.is_some(), "Migrations table not found");

            Ok(())
        }

        #[tokio::test]
        async fn invalid_url() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            assert!(matches!(
                migrate(
                    &conn,
                    String::from("https://jsonplaceholder.typicode.com/todos/1"),
                )
                .await,
                Err(LibsqlRemoteMigratorError::ReqwestError(_))
            ));

            Ok(())
        }

        #[tokio::test]
        async fn empty_url() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            assert!(matches!(
                migrate(&conn, String::from(""),).await,
                Err(LibsqlRemoteMigratorError::MigrationUrlNotValid(_))
            ));

            Ok(())
        }
    }

    mod migration {
        use super::super::*;
        use crate::migration_tests::setup_test_db;

        #[tokio::test]
        async fn check_for_valid_execution() -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(&conn, String::from("https://raw.githubusercontent.com/prashant1k99/libsql_migration/refs/heads/main/tests/remote-sql/v1.json")).await?;

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
                let type_name = row.get::<String>(2)?.to_uppercase();
                column_info.insert(name, type_name);
            }

            assert_eq!(
                column_info.get_key_value("id"),
                Some((&String::from("id"), &String::from("INTEGER")))
            );

            assert_eq!(
                column_info.get_key_value("status"),
                Some((&String::from("status"), &String::from("BOOLEAN")))
            );

            let mut rows = conn
                .query("PRAGMA table_info('test2');", libsql::params![])
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
                column_info.get_key_value("status"),
                Some((&String::from("status"), &String::from("BOOLEAN")))
            );

            Ok(())
        }

        #[tokio::test]
        async fn should_not_execute_new_script_with_same_id()
        -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(&conn, String::from("https://raw.githubusercontent.com/prashant1k99/libsql_migration/refs/heads/main/tests/remote-sql/v1.json")).await?;
            migrate(&conn, String::from("https://raw.githubusercontent.com/prashant1k99/libsql_migration/refs/heads/main/tests/remote-sql/v3.json")).await?;

            let mut rows = conn
                .query("PRAGMA table_info('test1');", libsql::params![])
                .await?;

            let mut column_info: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();

            while let Some(row) = rows.next().await? {
                let name = row.get::<String>(1)?;
                let type_name = row.get::<String>(2)?.to_uppercase();
                column_info.insert(name, type_name);
            }

            assert_eq!(column_info.get_key_value("Email"), None);

            Ok(())
        }

        #[tokio::test]
        async fn should_execute_correctly_for_multiple_urls()
        -> Result<(), Box<dyn std::error::Error>> {
            let (conn, _temp_dir) = setup_test_db().await?;

            migrate(&conn, String::from("https://raw.githubusercontent.com/prashant1k99/libsql_migration/refs/heads/main/tests/remote-sql/v1.json")).await?;
            migrate(&conn, String::from("https://raw.githubusercontent.com/prashant1k99/libsql_migration/refs/heads/main/tests/remote-sql/v2.json")).await?;

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
                let type_name = row.get::<String>(2)?.to_uppercase();
                column_info.insert(name, type_name);
            }

            assert_eq!(
                column_info.get_key_value("id"),
                Some((&String::from("id"), &String::from("INTEGER")))
            );

            assert_eq!(
                column_info.get_key_value("status"),
                Some((&String::from("status"), &String::from("BOOLEAN")))
            );

            assert_eq!(
                column_info.get_key_value("Email"),
                Some((&String::from("Email"), &String::from("TEXT")))
            );

            let mut rows = conn
                .query("PRAGMA table_info('test2');", libsql::params![])
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
                column_info.get_key_value("status"),
                Some((&String::from("status"), &String::from("BOOLEAN")))
            );

            assert_eq!(
                column_info.get_key_value("Email"),
                Some((&String::from("Email"), &String::from("TEXT")))
            );
            Ok(())
        }
    }
}
