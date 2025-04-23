use crate::errors::LibsqlRemoteMigratorError;
use crate::util::create_migration_table;
use libsql::Connection;

#[derive(serde::Deserialize, Debug)]
struct RemoteMigrationFileSchema {
    id: String,
    name: String,
    file: String,
}

async fn make_request(
    url: String,
) -> Result<Vec<RemoteMigrationFileSchema>, LibsqlRemoteMigratorError> {
    let mut files = reqwest::get(url)
        .await?
        .json::<Vec<RemoteMigrationFileSchema>>()
        .await?;

    files.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(files)
}

pub async fn migrate(conn: &Connection, url: String) -> Result<(), LibsqlRemoteMigratorError> {
    create_migration_table(conn).await?;

    // Make a Rest request to the URL
    let all_files = make_request(url).await?;
    println!("All files: {:?}", all_files);

    Ok(())
}
