use crate::errors::LibsqlContentMigratorError;
use crate::util::{MigrationResult, create_migration_table, execute_migration};
use libsql::Connection;

pub async fn migrate(
    conn: &Connection,
    migration_id: String,
    migration_script: String,
) -> Result<MigrationResult, LibsqlContentMigratorError> {
    if migration_id.is_empty() {
        return Err(LibsqlContentMigratorError::InvalidInput(
            "`migration_id` is empty".to_string(),
        ));
    }
    if migration_script.is_empty() {
        return Err(LibsqlContentMigratorError::InvalidInput(
            "`migration_script` is empty".to_string(),
        ));
    }

    create_migration_table(conn).await?;

    let res = execute_migration(conn, migration_id, migration_script).await?;

    Ok(res)
}
