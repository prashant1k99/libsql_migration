use crate::errors::LibsqlMigratorError;
use libsql::Connection;

pub fn migrate_using_link(conn: &Connection, url: String) -> Result<(), LibsqlMigratorError> {
    // Fetch the central endpoint and it should satisfy the URL serde condition
    Ok(())
}
