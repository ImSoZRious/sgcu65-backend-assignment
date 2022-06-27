CREATE TABLE tasks (
  id        SERIAL PRIMARY KEY,
  name      VARCHAR NOT NULL,
  content   TEXT NOT NULL,
  status    VARCHAR NOT NULL,
  timestamp VARCHAR NOT NULL
)