CREATE SEQUENCE users_seq;
CREATE SEQUENCE tasks_seq;

CREATE TABLE users (
  id        INTEGER DEFAULT nextval('users_seq') PRIMARY KEY NOT NULL,
  email     VARCHAR NOT NULL,
  firstname VARCHAR NOT NULL,
  lastname  VARCHAR NOT NULL,
  role      VARCHAR NOT NULL
);

CREATE TABLE tasks (
  id        INTEGER DEFAULT nextval('tasks_seq') PRIMARY KEY NOT NULL,
  name      VARCHAR NOT NULL,
  content   TEXT NOT NULL,
  status    VARCHAR NOT NULL,
  deadline VARCHAR NOT NULL
);

ALTER SEQUENCE users_seq RESTART WITH 10001;

ALTER SEQUENCE tasks_seq RESTART WITH 10001;