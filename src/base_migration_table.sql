CREATE TABLE IF NOT EXISTS libsql_migrations (
  id TEXT PRIMARY KEY,
  status BOOLEAN default true,
  exec_time DATE default current_timestamp
);
