-- Your SQL goes here
ALTER TABLE shows ADD COLUMN season BIGINT NOT NULL;
ALTER TABLE shows ADD COLUMN parent_season BIGINT NOT NULL;
ALTER TABLE episodes ADD COLUMN episode_number INTEGER;
