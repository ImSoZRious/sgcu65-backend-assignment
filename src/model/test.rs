// this module doesn't count as compilation unit, so it doesn't impact performance
// of binary in production
#[allow(dead_code)]
#[allow(unused_imports)]
use std::env;

use super::task::{NewTask, Task, UpdateTask};
use super::user::{NewUser, UpdateUser, User};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

macro_rules! s {
  ($s: expr) => {{
    $s.to_string()
  }};
}

fn get_db_con() -> PgConnection {
  use dotenv::dotenv;
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("No database env is set.")
    .to_string();

  let conn = PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url.to_string()));

  conn
}

#[test]
fn connection_test() {
  let _conn = get_db_con();
}

#[test]
fn get_all_user() {
  let conn = get_db_con();

  let _users = User::get_all(&conn).unwrap();
}

#[test]
fn create_user() {
  let conn = get_db_con();

  let new_user = NewUser {
    email: s!("idontlikesand@hotmail.com"),
    firstname: s!("Anakin"),
    lastname: s!("Skywalker"),
    role: s!("Frontend Developer"),
  };

  let _user = User::create(&new_user, &conn).unwrap();

  println!("{:?}", _user);
}

#[test]
fn update_user() {
  let conn = get_db_con();

  let update_info = UpdateUser {
    id: 1,
    email: Some(s!("i_dont_like_sand@hotmail.com")),
    firstname: None,
    lastname: None,
    role: None,
  };

  let _result = User::update(&update_info, &conn).unwrap();
}

#[test]
fn delete_user() {
  let conn = get_db_con();

  let delete_id = 3;

  let _del_count = User::delete(delete_id, &conn).unwrap();
}

#[test]
fn find_user() {
  let conn = get_db_con();

  let user_id = 3;

  let _user = User::find(user_id, &conn).unwrap();
}

// Task test
#[test]
fn get_all_task() {
  let conn = get_db_con();

  let _users = Task::get_all(&conn).unwrap();
}

#[test]
fn create_task() {
  let conn = get_db_con();

  let new_task = NewTask {
    name: s!("Comprog"),
    content: s!("Grader"),
    status: s!("In Progress"),
    deadline: s!("2022-10-31 12:00:00"),
  };

  let _task = Task::create(&new_task, &conn).unwrap();
}

#[test]
fn update_task() {
  let conn = get_db_con();

  let update_info = UpdateTask {
    id: 1,
    name: None,
    content: Some(s!("EZ Comprog")),
    status: Some(s!("Done")),
    deadline: None,
  };

  let _result = Task::update(&update_info, &conn).unwrap();
}

#[test]
fn delete_task() {
  let conn = get_db_con();

  let delete_id = 3;

  let _del_count = Task::delete(delete_id, &conn).unwrap();
}

#[test]
fn find_task() {
  let conn = get_db_con();

  let task_id = 3;

  let _task = Task::find(task_id, &conn).unwrap();
}
