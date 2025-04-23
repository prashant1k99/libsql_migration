use crate::errors::LibsqlRemoteMigratorError;
use crate::util::{MigrationResult, create_migration_table, execute_migration};
use libsql::Connection;

#[derive(serde::Deserialize, Debug)]
struct RemoteMigrationFileSchema {
    id: String,
    url: String,
}

async fn make_request(
    url: String,
) -> Result<Vec<RemoteMigrationFileSchema>, LibsqlRemoteMigratorError> {
    let mut files = reqwest::get(url)
        .await?
        .json::<Vec<RemoteMigrationFileSchema>>()
        .await?;

    files.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(files)
}

async fn get_file_content(url: String) -> Result<String, LibsqlRemoteMigratorError> {
    let content = reqwest::get(url).await?.text().await?;

    Ok(content)
}

pub async fn migrate(conn: &Connection, url: String) -> Result<bool, LibsqlRemoteMigratorError> {
    create_migration_table(conn).await?;

    let all_files = make_request(url).await?;

    let mut did_new_migration = false;
    for file in all_files {
        let content = get_file_content(file.url).await?;
        if let MigrationResult::Executed = execute_migration(conn, file.id, content).await? {
            did_new_migration = true
        }
    }

    Ok(did_new_migration)
}
