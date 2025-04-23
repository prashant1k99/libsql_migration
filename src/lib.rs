//! # LibSQL Migration Crate
//!
//! `libsql_migration` is a Rust crate designed to manage database migrations
//! for LibSQL (and SQLite). It provides a simple and flexible way to apply
//! SQL migration scripts to your database, ensuring that schema changes are
//! tracked and applied consistently.
//!
//! ## Key Features
//!
//! *   **Multiple Migration Sources:** Supports migrations from local directories (`dir` feature),
//!     embedded SQL content (`content` feature), or remote HTTP endpoints (`remote` feature).
//! *   **Automatic Tracking:** Creates and manages a `libsql_migrations` table to keep
//!     track of which migrations have already been applied.
//! *   **Simple API:** Offers straightforward functions (`migrate`) within each feature module
//!     to apply migrations.
//! *   **Async Support:** Built with `async/await` for non-blocking database operations.
//! *   **Feature-Gated:** Use only the features you need to keep dependencies minimal.
//!
//! ## Getting Started
//!
//! Add `libsql_migration` to your `Cargo.toml`. By default, the `dir` feature is enabled.
//!
//! ```toml
//! [dependencies]
//! libsql_migration = "0.1.0" # Replace with the latest version
//! libsql = { version = "...", features = ["local"] } # Or other libsql features
//! tokio = { version = "1", features = ["full"] } # For the async runtime
//! ```
//!
//! If you want to use other features like `content` or `remote`, you might need to
//! disable default features and enable the specific ones you need. See the "Features"
//! section below.
//!
//! ## Migration Tracking
//!
//! The migrator automatically creates a `libsql_migrations` table in your database
//! (if it doesn't exist) upon the first migration attempt. This table stores the
//! unique identifiers (e.g., filenames for `dir`, provided IDs for `content`/`remote`)
//! of the migrations that have been successfully applied. Before applying any
//! migration, the crate checks this table to prevent reapplying already executed scripts.
//!
//! ## Features
//!
//! This crate provides different ways to source migrations, controlled by Cargo features:
//!
//! *   **`dir`** (enabled by default): Migrates using SQL files from a local directory.
//!     See the [`dir`] module documentation for details and usage examples.
//! *   **`content`**: Migrates using SQL content embedded directly in your code or loaded
//!     dynamically. See the [`content`] module documentation for details and usage examples.
//!     Requires disabling default features.
//! *   **`remote`**: Migrates using SQL files fetched from a remote location (e.g., HTTP).
//!     See the [`remote`] module documentation for details and usage examples.
//!     Requires disabling default features and adding `reqwest` and `serde` as dependencies.
//!
//! Enable features in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! # Only enable the 'content' feature
//! libsql_migration = { version = "...", default-features = false, features = ["content"] }
//!
//! # Enable 'remote' feature (requires reqwest and serde)
//! # libsql_migration = { version = "...", default-features = false, features = ["remote"] }
//! # reqwest = { version = "...", features = ["json"] }
//! # serde = { version = "...", features = ["derive"] }
//!
//! # Enable 'dir' and 'remote' features (dir is default, so just adding remote works too if default is kept)
//! # libsql_migration = { version = "...", features = ["remote"] } # If default-features = true (default)
//! # libsql_migration = { version = "...", default-features = false, features = ["dir", "remote"] } # Explicit
//! ```
//!
//! ## Example Usage (using default `dir` feature)
//!
//! For detailed examples of each feature, please refer to the respective module documentation
//! ([`dir`], [`content`], [`remote`]). A basic example using the default `dir` feature is shown
//! in the [`dir`] module documentation.
//!
//! [GitHub Repository](https://github.com/prashant1k99/libsql_migration)

pub mod errors;
pub mod util;

#[cfg(feature = "content")]
pub mod content;

#[cfg(feature = "remote")]
pub mod remote;

#[cfg(feature = "dir")]
pub mod dir;
