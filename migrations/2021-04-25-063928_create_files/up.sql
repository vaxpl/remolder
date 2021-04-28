CREATE TABLE files (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  hash VARCHAR(64) NOT NULL,
  path VARCHAR(256) NOT NULL,
  mime VARCHAR(64) NOT NULL,
  size INTEGER NOT NULL,
  createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)
