-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title varchar not null,
    slug VARCHAR NOT NULL,
    body TEXT NOT NULL
)