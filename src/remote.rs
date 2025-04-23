use crate::errors::LibsqlRemoteMigratorError;
use libsql::Connection;

pub fn migrate(_conn: &Connection, _url: String) -> Result<(), LibsqlRemoteMigratorError> {
    // Fetch the central endpoint and it should satisfy the URL serde condition
    Ok(())
}
