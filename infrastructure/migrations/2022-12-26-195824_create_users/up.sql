-- up.sql
CREATE TABLE users (
    "id"                  SERIAL PRIMARY KEY,
    "username"            VARCHAR(256) NOT NULL, -- !TODO figure out a good length
    "nickname"            VARCHAR(256) NOT NULL,
    "password_hash"       VARCHAR(512) NOT NULL, -- !TODO figure out hash size 
    -- email VARCHAR(320) NOT NULL, -- max valid email length
    "created_at"          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- !TODO more socials
    "twitter_username"    VARCHAR(256),
    "instagram_username"  VARCHAR(256),
    "tiktok_username"     VARCHAR(256),
    "onlyfans_username"   VARCHAR(256),
    "snapchat_username"   VARCHAR(256),
    "youtube_username"    VARCHAR(256)

    -- maybe myanime list etc
);

CREATE TABLE user_followers(
    "user_id"       INT NOT NULL REFERENCES users(id),
    "follower_id"   INT NOT NULL REFERENCES users(id),
    
    PRIMARY KEY ("user_id", "follower_id")
);