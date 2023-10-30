-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_follows(
    "user_id"           INT NOT NULL REFERENCES users(id),
    "following_id"      INT NOT NULL REFERENCES users(id),
    
    PRIMARY KEY ("user_id", "following_id")
);

CREATE TABLE IF NOT EXISTS user_blocked(
    "user_id"           INT NOT NULL REFERENCES users(id),
    "blocked_id"        INT NOT NULL REFERENCES users(id),
    
    PRIMARY KEY ("user_id", "blocked_id")
);