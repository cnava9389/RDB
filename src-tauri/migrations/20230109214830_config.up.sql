-- Add up migration script here
CREATE TABLE IF NOT EXISTS Config (
    id BLOB PRIMARY KEY UNIQUE NOT NULL,
    cache BOOLEAN NOT NULL DEFAULT 0,
    ttl INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);