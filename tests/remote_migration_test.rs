use libsql_migration::remote::migrate;

#[cfg(test)]
mod migration_tests {
    use tempfile::tempdir;

    async fn setup_test_db() -> Result<libsql::Connection, Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let db = libsql::Builder::new_local(db_path).build().await?;
        let conn = db.connect()?;

        Ok(conn)
    }

    mod base {
        use crate::migration_tests::setup_test_db;

        use super::super::*;

        #[tokio::test]
        async fn establish_connection() -> Result<(), Box<dyn std::error::Error>> {
            let conn = setup_test_db().await?;

            migrate(&conn, String::from("localhost::8080/")).await?;
            Ok(())
        }
    }
}
