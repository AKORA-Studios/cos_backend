-- up.sql
CREATE TABLE messages (
    id          SERIAL PRIMARY KEY,
    content     VARCHAR(1024) NOT NULL,
    author_id   INT NOT NULL REFERENCES users(id),
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);