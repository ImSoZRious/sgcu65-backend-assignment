CREATE TABLE users (
  id        SERIAL PRIMARY KEY,
  email     VARCHAR NOT NULL,
  firstname VARCHAR NOT NULL,
  lastname  VARCHAR NOT NULL,
  role      VARCHAR NOT NULL
)