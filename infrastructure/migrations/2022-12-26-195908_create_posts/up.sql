-- up.sql
CREATE TABLE posts (
    id              SERIAL PRIMARY KEY,
    description     VARCHAR(1024) NOT NULL,
    author_id       INT NOT NULL REFERENCES users(id),

    downloads       INT NOT NULL DEFAULT 0,
    likes           INT NOT NULL DEFAULT 0,

    tags            TEXT ARRAY NOT NULL,
    photographer_id INT REFERENCES users(id),
    -- visible people in post_people table
    
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE post_people (
    id              SERIAL PRIMARY KEY,
    post_id         INT NOT NULL REFERENCES posts(id),
    person_id       INT NOT NULL REFERENCES users(id)
);