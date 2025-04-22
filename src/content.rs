use crate::errors;
use crate::util;
use libsql::Connection;

pub async fn run_migration(
    conn: &Connection,
    migration_id: String,
    migration_script: String,
) -> Result<(), errors::LibsqlMigratorError> {
    util::create_migration_table(conn).await?;

    let mut stmt = conn
        .prepare("SELECT status FROM libsql_migrations WHERE id = ?;")
        .await?;

    let mut rows = stmt.query([migration_id.clone()]).await?;

    if let Some(record) = rows.next().await? {
        let status_value = record.get_value(0)?;
        if let libsql::Value::Integer(1) = status_value {
            return Ok(());
        }
    }
    conn.execute(
        "INSERT INTO libsql_migrations (id) VALUES (?) ON CONFLICT(id) DO NOTHING",
        libsql::params![migration_id.clone()],
    )
    .await?;

    conn.execute(&migration_script, libsql::params!()).await?;

    conn.execute(
        "UPDATE libsql_migrations SET status = true, exec_time = CURRENT_TIMESTAMP WHERE id = ?",
        libsql::params![migration_id],
    )
    .await?;
    Ok(())
}
