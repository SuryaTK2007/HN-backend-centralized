-- Add migration script here
ALTER TABLE notes ADD COLUMN user_id TEXT NOT NULL DEFAULT '';
