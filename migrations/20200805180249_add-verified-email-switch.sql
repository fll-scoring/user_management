-- Add migration script here
ALTER TABLE users ADD COLUMN is_verified BOOLEAN DEFAULT false NOT NULL
