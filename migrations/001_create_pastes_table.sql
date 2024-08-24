-- Add migration script here
CREATE TABLE pastes (
    id VARCHAR(8) PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);