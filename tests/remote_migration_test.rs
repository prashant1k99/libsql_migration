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
}
