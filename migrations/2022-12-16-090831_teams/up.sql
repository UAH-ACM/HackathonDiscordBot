-- Your SQL goes here
CREATE TABLE teams (
  id bigint PRIMARY KEY,
  team_name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  leader TEXT NOT NULL,
  members TEXT NOT NULL,
  points bigint NOT NULL
);