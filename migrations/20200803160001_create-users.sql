-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  userid uuid PRIMARY KEY,
  email TEXT NOT NULL,
  pw_hash TEXT NOT NULL
);
