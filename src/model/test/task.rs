use super::super::task::{NewTask, PartialTask, Task};
use super::get_db_con;
use super::s;

#[test]
fn get_all_task() {
  let conn = get_db_con();

  let _tasks = Task::get_all(&conn).unwrap();

  println!("{:?}", _tasks);
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

  println!("{:?}", _task);
}

#[test]
fn update_task() {
  let conn = get_db_con();

  let id = Task::get_all(&conn).unwrap()[0].id;

  let update_info = PartialTask {
    id: Some(id),
    name: Some(s!("Calculus")),
    ..Default::default()
  };

  let _result = Task::update(&update_info, &conn).unwrap();
}

#[test]
fn find_task() {
  let conn = get_db_con();

  let task_id = Task::get_all(&conn).unwrap()[0].id;

  let _task = Task::find(task_id, &conn).unwrap();

  println!("{:?}", _task);
}

#[test]
fn find_task_by_name() {
  let conn = get_db_con();
  let task_name = "Cal";

  let q = PartialTask {
    name: Some(s!(task_name)),
    ..Default::default()
  };

  let _task = Task::query(&q, &conn).unwrap();

  println!("{:?}", _task);
}

#[test]
fn delete_task() {
  let conn = get_db_con();

  let delete_id = Task::get_all(&conn).unwrap()[0].id;

  let _del_count = Task::delete(delete_id, &conn).unwrap();
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

  let id = Task::get_all(&conn).unwrap()[0].id;

  let edit_infos = vec![PartialTask {
    id: Some(id),
    name: Some(s!("yay")),
    content: Some(s!("im running out of name")),
    ..Default::default()
  }];

  for edit_info in edit_infos.iter() {
    let _ = Task::update(edit_info, &conn).unwrap();
  }

  let users = Task::get_all(&conn).unwrap();
  let first = users.len();

  let _del = Task::delete(id, &conn);
  let users = Task::get_all(&conn).unwrap();
  assert_eq!(users.len(), first - 1);
}
