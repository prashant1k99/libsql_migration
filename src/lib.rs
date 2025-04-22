//! `libsql_migration` provides a simple migration mechanism for libsql databases.
//!
//! It allows you to apply SQL migration files located in a specified directory
//! to your database, keeping track of which migrations have already been applied.
//!
//! # Usage
//!
//! **Note:** This crate relies on the `tokio` runtime for its asynchronous operations. Ensure `tokio` is included in your project dependencies and an appropriate runtime is available (e.g., using `#[tokio::main]`).
//!
//! ```no_run
//! use libsql_migration::{dir::migrate, errors::LibsqlMigratorError};
//! use libsql::Builder;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), LibsqlMigratorError> {
//!     // Connect to your database
//!     let db = Builder::new_local("my_database.db").build().await.unwrap();
//!     let conn = db.connect().unwrap();
//!
//!     // Specify the path to your migration files
//!     let migrations_folder = PathBuf::from("./migrations");
//!
//!     // Run the migrations
//!     match migrate(&conn, migrations_folder).await {
//!         Ok(applied) => {
//!             if applied {
//!                 println!("Migrations applied successfully.");
//!             } else {
//!                 println!("No new migrations to apply.");
//!             }
//!         }
//!         Err(e) => {
//!             eprintln!("Migration failed: {}", e);
//!             return Err(e);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Migration Files
//!
//! Migration files should be plain SQL files (`.sql` extension) located within the
//! specified migrations directory. They are executed in lexicographical order based
//! on their filenames. It's recommended to prefix filenames with numbers or timestamps
//! to ensure correct ordering (e.g., `0001_initial.sql`, `0002_add_users_table.sql`).
//!
//! The migrator creates a `libsql_migrations` table in your database to track
//! applied migrations.
//!
//! [GitHub Repository](https://github.com/prashant1k99/libsql_migration)

pub mod errors;
pub(crate) mod util;

#[cfg(feature = "content")]
pub mod content;

#[cfg(feature = "cloud")]
pub mod cloud;

#[cfg(feature = "dir")]
pub mod dir;
