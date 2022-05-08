CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  href VARCHAR NOT NULL,
  github VARCHAR NOT NULL,
  picture text NOT NULL
)