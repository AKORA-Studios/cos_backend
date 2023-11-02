-- up.sql
CREATE TABLE posts (
    "id"              SERIAL PRIMARY KEY,
    "caption"         VARCHAR(256),
    "description"     VARCHAR(1024),
    "user_id"         INT NOT NULL REFERENCES users(id),

    "tags"            TEXT ARRAY NOT NULL,
    "photographer_id" INT REFERENCES users(id),
    -- visible people in post_people table

    -- geo location
    "lat"             DOUBLE PRECISION,
    "lon"             DOUBLE PRECISION,
    
    "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE post_depicted_people (
    "post_id"         INT NOT NULL REFERENCES posts(id),
    "user_id"         INT NOT NULL REFERENCES users(id),

    PRIMARY KEY ("post_id", "user_id")
);

CREATE TABLE post_likes(
    "user_id" INT NOT NULL REFERENCES users(id),
    "post_id" INT NOT NULL REFERENCES posts(id),
    
    PRIMARY KEY ("user_id", "post_id")
);

CREATE TABLE post_downloads(
    "user_id" INT NOT NULL REFERENCES users(id),
    "post_id" INT NOT NULL REFERENCES posts(id),
    
    PRIMARY KEY ("user_id", "post_id")
);