-- Your SQL goes here
CREATE TABLE IF NOT EXISTS urls
(
    id         SERIAL PRIMARY KEY,
    long_url   TEXT NOT NULL,
    short_url  VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);