-- Add up migration script here
CREATE TABLE IF NOT EXISTS DbUser (
    id BLOB PRIMARY KEY UNIQUE NOT NULL,
    relation_ids BLOB NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);