CREATE TABLE IF NOT EXISTS test_migration (
  id TEXT PRIMARY KEY,
  status BOOLEAN default true,
  exec_time DATE default current_timestamp
);
