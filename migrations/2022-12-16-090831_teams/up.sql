-- Your SQL goes here
CREATE TABLE teams (
  id bigint PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  members TEST NOT NULL,
  points bigint NOT NULL
);