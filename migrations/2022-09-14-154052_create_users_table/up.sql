CREATE TABLE users (
  uuid UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  age SMALLINT NOT NULL,
  is_sub BOOLEAN NOT NULL
);