CREATE TABLE IF NOT EXISTS libsql_migrations (
  id TEXT PRIMARY KEY,
  status BOOLEAN default false,
  exec_time DATE 
);
