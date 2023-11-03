-- up.sql

CREATE TYPE content_type AS ENUM ('image', 'video', 'audio');

CREATE TABLE attachments (
    "id"              SERIAL PRIMARY KEY,
    
    "url"             VARCHAR(1024) NOT NULL,
    "content_type"    content_type NOT NULL,

    "created_at"      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE messages (
    id              SERIAL PRIMARY KEY,
    content         VARCHAR(1024) NOT NULL,

    attachment_id   INT REFERENCES attachments(id),
    reply_to        INT REFERENCES messages(id),

    from_id         INT NOT NULL REFERENCES users(id),
    to_id           INT NOT NULL REFERENCES users(id),

    created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX message_author ON messages (from_id);
CREATE INDEX message_receiver ON messages (to_id);