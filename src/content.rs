use crate::errors::LibsqlMigratorBaseError as LibsqlContentMigratorError;
use crate::util::{MigrationResult, create_migration_table, execute_migration};
use libsql::Connection;

pub async fn migrate(
    conn: &Connection,
    migration_id: String,
    migration_script: String,
) -> Result<MigrationResult, LibsqlContentMigratorError> {
    create_migration_table(conn).await?;

    execute_migration(conn, migration_id, migration_script).await
}
