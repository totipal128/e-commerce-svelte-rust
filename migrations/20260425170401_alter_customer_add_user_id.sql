-- Add migration script here
ALTER TABLE customer ADD COLUMN user_id INTEGER REFERENCES users(id) ON DELETE SET NULL DEFAULT NULL;
