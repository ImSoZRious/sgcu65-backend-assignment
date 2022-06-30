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
  deadline  VARCHAR NOT NULL
);

CREATE TABLE users_tasks (
  user_id INTEGER NOT NULL,
  task_id INTEGER NOT NULL,
  CONSTRAINT PK_UserTask PRIMARY KEY
  (
    user_id,
    task_id
  ),
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
  FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE
);

ALTER SEQUENCE users_seq RESTART WITH 10001;

ALTER SEQUENCE tasks_seq RESTART WITH 10001;