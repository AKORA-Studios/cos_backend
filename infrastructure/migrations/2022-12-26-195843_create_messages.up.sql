-- up.sql

CREATE TYPE IF NOT EXISTS content_type AS ENUM ('image', 'video', 'audio');

CREATE TABLE IF NOT EXISTS attachments (
    "id"              SERIAL PRIMARY KEY,
    
    "url"             VARCHAR(1024) NOT NULL,
    "content_type"    content_type NOT NULL,

    "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS messages (
    id              SERIAL PRIMARY KEY,
    content         VARCHAR(1024) NOT NULL,

    attachment_id   INT REFERENCES attachments(id),
    reply_to        INT REFERENCES messages(id),

    from_id         INT NOT NULL REFERENCES users(id),
    to_id           INT NOT NULL REFERENCES users(id),

    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS message_author ON messages (from_id);
CREATE INDEX IF NOT EXISTS message_receiver ON messages (to_id);