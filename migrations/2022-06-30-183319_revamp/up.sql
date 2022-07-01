CREATE SEQUENCE users_seq;
CREATE SEQUENCE tasks_seq;
CREATE SEQUENCE teams_seq;

CREATE TABLE "teams" (
  "id" INTEGER DEFAULT nextval('teams_seq') PRIMARY KEY NOT NULL,
  "name" VARCHAR NOT NULL
);

CREATE TABLE "tasks" (
  "id" INTEGER DEFAULT nextval('tasks_seq') PRIMARY KEY NOT NULL,
  "name" VARCHAR NOT NULL,
  "content" VARCHAR NOT NULL,
  "status" VARCHAR NOT NULL,
  "deadline" VARCHAR NOT NULL,
  "owner_team_id" INTEGER
);

CREATE TABLE "users" (
  "id" INTEGER DEFAULT nextval('users_seq') PRIMARY KEY NOT NULL,
  "firstname" VARCHAR NOT NULL,
  "lastname" VARCHAR NOT NULL,
  "email" VARCHAR NOT NULL,
  "role" VARCHAR NOT NULL,
  "team_id" INTEGER,
  "pwd_hash" VARCHAR NOT NULL,
  "permission" VARCHAR NOT NULL,
  FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE SET NULL
);

CREATE TABLE "session" (
  "id" SERIAL NOT NULL PRIMARY KEY,
  "api_key" VARCHAR NOT NULL,
  "expire_time" VARCHAR NOT NULL,
  "user_id" INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

ALTER SEQUENCE users_seq RESTART WITH 10001;

ALTER SEQUENCE tasks_seq RESTART WITH 10001;

ALTER SEQUENCE teams_seq RESTART WITH 10001;
