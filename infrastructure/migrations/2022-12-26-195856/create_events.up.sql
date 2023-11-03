-- up.sql
CREATE TABLE events (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(256) NOT NULL,
    start_time  TIMESTAMPTZ NOT NULL, -- !TODO timezones
    end_time    TIMESTAMPTZ NOT NULL, -- !TODO timezones

    lat         DOUBLE PRECISION NOT NULL,
    lon         DOUBLE PRECISION NOT NULL

    -- !TODO cover image
);