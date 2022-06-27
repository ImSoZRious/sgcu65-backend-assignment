CREATE TABLE users (
  id        SERIAL PRIMARY KEY,
  email     VARCHAR NOT NULL,
  firstname VARCHAR NOT NULL,
  lastname  VARCHAR NOT NULL,
  role      VARCHAR NOT NULL
);

CREATE TABLE tasks (
  id        SERIAL PRIMARY KEY,
  name      VARCHAR NOT NULL,
  content   TEXT NOT NULL,
  status    VARCHAR NOT NULL,
  deadline VARCHAR NOT NULL
);