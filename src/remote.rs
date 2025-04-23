use crate::errors::LibsqlMigratorBaseError;
use libsql::Connection;

pub fn migrate(conn: &Connection, url: String) -> Result<(), LibsqlMigratorBaseError> {
    // Fetch the central endpoint and it should satisfy the URL serde condition
    Ok(())
}
