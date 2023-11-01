-- up.sql
CREATE TABLE comments (
    "id"          SERIAL PRIMARY KEY,
    "content"     VARCHAR(1024) NOT NULL,
    
    "user_id"     INT NOT NULL REFERENCES users(id),
    "post_id"     INT NOT NULL REFERENCES posts(id),

    -- if it is a reply to another comment
    "reply_to"    INT REFERENCES comments(id),

    "upvotes"     INT NOT NULL DEFAULT 0,
    
    "created_at"  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);