// this module doesn't count as compilation unit, so it doesn't impact performance
// of binary in production
#[allow(dead_code)]
#[allow(unused_imports)]
use std::env;

use super::task::{NewTask, Task, TaskQuery, UpdateTask};
use super::user::{NewUser, UpdateUser, User, UserQuery};

use diesel::pg::PgConnection;
use diesel::prelude::*;

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

#[test]
fn find_user_by_attribute() {
  let conn = get_db_con();

  // let user1_firstname = "Stranger";
  let user1_lastname = "Thin";

  // let user2_firstname = "Spir";
  // let user2_lastname = "Awa";

  let q = UserQuery {
    firstname: None,
    lastname: Some(s!(user1_lastname)),
  };

  let _user = User::query(&q, &conn).unwrap();
  println!("{:?}", _user);
}

#[test]
fn combined_user_test() {
  let conn = get_db_con();

  let users = vec![
    NewUser {
      email: s!("abc@hotmail.com"),
      firstname: s!("Spirited"),
      lastname: s!("Away"),
      role: s!("UX/UI Designer"),
    },
    NewUser {
      email: s!("def@outlook.com"),
      firstname: s!("Inter"),
      lastname: s!("Stellar"),
      role: s!("Backend Developer"),
    },
    NewUser {
      email: s!("noidea@gmail.com"),
      firstname: s!("Get"),
      lastname: s!("Out"),
      role: s!("Project Manager"),
    },
  ];

  for user in users.iter() {
    let _user = User::create(user, &conn).unwrap();
  }

  let edit_infos = vec![UpdateUser {
    id: 10003,
    email: None,
    firstname: Some(s!("God")),
    lastname: Some(s!("Father")),
    role: None,
  }];

  for edit_info in edit_infos.iter() {
    let _ = User::update(edit_info, &conn).unwrap();
  }

  let users = User::get_all(&conn).unwrap();
  assert_eq!(users.len(), 3);

  let _del = User::delete(10002, &conn);
  let users = User::get_all(&conn).unwrap();
  assert_eq!(users.len(), 2);
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

#[test]
fn find_task_by_name() {
  let conn = get_db_con();
  let task_name = "ComProg";

  let q = TaskQuery {
    name: Some(s!(task_name)),
  };

  let _task = Task::query(&q, &conn).unwrap();

  println!("{:?}", _task);
}

#[test]
fn combined_task_test() {
  let conn = get_db_con();

  let users = vec![
    NewTask {
      name: s!("Nono"),
      content: s!("nop"),
      status: s!("Done"),
      deadline: s!("2022-28-06 19:06:00"),
    },
    NewTask {
      name: s!("Calculus"),
      content: s!("read your book"),
      status: s!("Done"),
      deadline: s!("2022-05-31 04:00:00"),
    },
    NewTask {
      name: s!("Chem lab"),
      content: s!("Info in the MCV"),
      status: s!("Done"),
      deadline: s!("2023-01-01 00:00:00"),
    },
  ];

  for task in users.iter() {
    let _task = Task::create(task, &conn).unwrap();
  }

  let edit_infos = vec![UpdateTask {
    id: 10003,
    name: Some(s!("yay")),
    content: Some(s!("im running out of name")),
    status: None,
    deadline: None,
  }];

  for edit_info in edit_infos.iter() {
    let _ = Task::update(edit_info, &conn).unwrap();
  }

  let users = Task::get_all(&conn).unwrap();
  assert_eq!(users.len(), 3);

  let _del = Task::delete(10002, &conn);
  let users = Task::get_all(&conn).unwrap();
  assert_eq!(users.len(), 2);
}
