-- Add migration script here
ALTER TABLE users ADD COLUMN admin BOOLEAN DEFAULT false;
