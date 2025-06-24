-- Add migration script here
CREATE TABLE users (
    id TEXT PRIMARY KEY,                  -- UUID
    username TEXT UNIQUE NOT NULL,        -- unique username
    password_hash TEXT NOT NULL,          -- hashed password
    created_at TEXT NOT NULL              -- timestamp
);
