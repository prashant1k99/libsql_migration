# libsql_migration

`libsql_migration` provides a simple migration mechanism for libsql databases. This crate is designed to help developers manage database migrations efficiently by offering support for directory-based migrations, remote migrations, and content-based migrations. Additionally, it tracks the status of applied migrations.

## Features

1. **Directory-based Migrations** (`dir` feature):

   - Apply SQL migration files from a specified directory.
   - Ensures migrations are executed in lexicographical order, based on filenames.
   - Tracks applied migrations in a `libsql_migrations` table.

2. **Content-based Migrations** (`content` feature):

   - Apply a single SQL migration script provided as a string.
   - Validate inputs to prevent empty migration IDs or scripts.

3. **Remote Migrations** (`remote` feature):

   - Fetch and apply migrations from a remote source (e.g., a URL).
   - Supports sorting and executing migrations in the correct order.

4. **Error Handling**:

   - Handles errors gracefully with detailed error messages.
   - Provides specific error types for different migration contexts (e.g., directory, content, and remote).

5. **Migration Tracking**:

   - Maintains a `libsql_migrations` table to track the status of migrations, including whether they were executed successfully.

6. **Asynchronous Execution**:
   - Leverages the `tokio` runtime for asynchronous operations, ensuring high performance.

---

## Installation

To use `libsql_migration`, add the following to your `Cargo.toml`:

```toml
[dependencies]
libsql_migration = { git = "https://github.com/prashant1k99/libsql_migration.git" }
tokio = { version = "1", features = ["full"] }
libsql = "0.1" # Replace with the appropriate version
```

---

## Usage

### Directory-based Migrations

The `dir` feature allows you to apply migrations from a directory containing `.sql` files.

```rust
use libsql_migration::dir::migrate;
use libsql_migration::errors::LibsqlDirMigratorError;
use libsql::Builder;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), LibsqlDirMigratorError> {
    // Connect to your database
    let db = Builder::new_local("my_database.db").build().await.unwrap();
    let conn = db.connect().unwrap();

    // Specify the path to your migration files
    let migrations_folder = PathBuf::from("./migrations");

    // Run the migrations
    match migrate(&conn, migrations_folder).await {
        Ok(applied) => {
            if applied {
                println!("Migrations applied successfully.");
            } else {
                println!("No new migrations to apply.");
            }
        }
        Err(e) => {
            eprintln!("Migration failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
```

#### Behavior:

- Files are executed in lexicographical order.
- Files must have a `.sql` extension.
- A `libsql_migrations` table is created to track applied migrations.

---

### Content-based Migrations

The `content` feature allows you to execute a migration script provided as a string.

```rust
use libsql_migration::content::migrate;
use libsql_migration::errors::LibsqlContentMigratorError;
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), LibsqlContentMigratorError> {
    // Connect to your database
    let db = Builder::new_local("my_database.db").build().await.unwrap();
    let conn = db.connect().unwrap();

    // Define the migration script and its ID
    let migration_id = "20230423_initial_setup".to_string();
    let migration_script = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
    "#.to_string();

    // Run the migration
    match migrate(&conn, migration_id, migration_script).await {
        Ok(result) => println!("Migration result: {:?}", result),
        Err(e) => eprintln!("Migration failed: {}", e),
    }

    Ok(())
}
```

#### Behavior:

- Validates that `migration_id` and `migration_script` are not empty.
- Executes the migration and updates the `libsql_migrations` table.

---

### Remote Migrations

The `remote` feature allows you to fetch and apply migrations from a remote source.

```rust
use libsql_migration::remote::migrate;
use libsql_migration::errors::LibsqlRemoteMigratorError;
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), LibsqlRemoteMigratorError> {
    // Connect to your database
    let db = Builder::new_local("my_database.db").build().await.unwrap();
    let conn = db.connect().unwrap();

    // Define the remote URL for migrations
    let remote_url = "https://example.com/migrations".to_string();

    // Run the migrations
    match migrate(&conn, remote_url).await {
        Ok(applied) => {
            if applied {
                println!("Migrations applied successfully.");
            } else {
                println!("No new migrations to apply.");
            }
        }
        Err(e) => eprintln!("Migration failed: {}", e),
    }

    Ok(())
}
```

#### Behavior:

- Fetches a list of migration files from the provided URL.
- Executes migrations in order and updates the `libsql_migrations` table.

---

## Migration Files

### Directory-based Migrations

- Files should be plain `.sql` files.
- Located in a specified folder.
- Executed in lexicographical order.

Example file structure:

```
migrations/
├── 0001_initial.sql
├── 0002_add_users_table.sql
├── 0003_add_orders_table.sql
```

### Remote Migrations

- Remote migrations should follow a structure where each migration file has an `id` and `url`.
- Example JSON response for remote migrations:

```json
[
  { "id": "0001_initial", "url": "https://example.com/0001_initial.sql" },
  {
    "id": "0002_add_users_table",
    "url": "https://example.com/0002_add_users_table.sql"
  }
]
```

---

## Error Handling

### Errors in `dir` Migrations

- `MigrationDirNotFound`: The specified directory does not exist.
- `InvalidMigrationPath`: The provided path is not valid.
- `ErrorWhileGettingSQLFiles`: Error occurred while traversing the folder.

### Errors in `content` Migrations

- `InvalidInput`: Either `migration_id` or `migration_script` is empty.
- `BaseError`: Underlying `libsql` error.

### Errors in `remote` Migrations

- `MigrationUrlNotValid`: The provided URL is invalid.
- `ReqwestError`: Error occurred during HTTP request.
- `BaseError`: Underlying `libsql` error.

---

## Design Details

### `libsql_migrations` Table

The migration tracking table is created automatically if it does not exist.

```sql
CREATE TABLE IF NOT EXISTS libsql_migrations (
  id TEXT PRIMARY KEY,
  status BOOLEAN DEFAULT false,
  exec_time DATE
);
```

- `id`: Unique identifier for the migration.
- `status`: Indicates whether the migration was executed successfully.
- `exec_time`: Timestamp of execution.

---

## Development

### Running Tests

Add tests to validate the library functionality. Use the following command to run the tests:

```bash
cargo test
```

---

## Contributing

Contributions are welcome! If you have suggestions or find a bug, please open an issue or submit a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

---

## Acknowledgments

Special thanks to the open-source community for their contributions and support.

---

## [GitHub Repository](https://github.com/prashant1k99/libsql_migration)
