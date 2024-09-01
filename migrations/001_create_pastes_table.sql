-- Add migration script here
CREATE TABLE pastes (
    id VARCHAR(8) PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    burn_after_read BOOLEAN NOT NULL DEFAULT FALSE,
    display_format VARCHAR(10) NOT NULL DEFAULT 'PlainText'
);