use super::super::user::{NewUser, PartialUser, User};
use super::get_db_con;
use super::s;

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
    pwd: s!("What sup"),
  };

  let _user = User::create(&new_user, &conn).unwrap();

  println!("{:?}", _user);
}

#[test]
fn update_user() {
  let conn = get_db_con();

  let id = User::get_all(&conn).unwrap()[0].id;

  let update_info = PartialUser {
    id: Some(id),
    email: Some(s!("i_dont_like_sand@hotmail.com")),
    ..Default::default()
  };

  let _result = User::update(&update_info, &conn).unwrap();
}

#[test]
fn delete_user() {
  let conn = get_db_con();

  let delete_id = User::get_all(&conn).unwrap()[0].id;

  let _del_count = User::delete(delete_id, &conn).unwrap();
}

#[test]
fn find_user() {
  let conn = get_db_con();

  let user_id = User::get_all(&conn).unwrap()[0].id;

  let _user = User::find(user_id, &conn).unwrap();

  println!("{:?}", _user);
}

#[test]
fn find_user_by_attribute() {
  let conn = get_db_con();

  // let user1_firstname = "Ana";
  let user1_lastname = "Skyw";

  // let user2_firstname = "Spir";
  // let user2_lastname = "Awa";

  let q = PartialUser {
    lastname: Some(s!(user1_lastname)),
    ..Default::default()
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
      pwd: s!("YO"),
    },
    NewUser {
      email: s!("def@outlook.com"),
      firstname: s!("Inter"),
      lastname: s!("Stellar"),
      role: s!("Backend Developer"),
      pwd: s!("123456"),
    },
    NewUser {
      email: s!("noidea@gmail.com"),
      firstname: s!("Get"),
      lastname: s!("Out"),
      role: s!("Project Manager"),
      pwd: s!("123456"),
    },
  ];

  for user in users.iter() {
    let _user = User::create(user, &conn).unwrap();
  }

  let id = User::get_all(&conn).unwrap()[0].id;

  let edit_infos = vec![PartialUser {
    id: Some(id),
    email: Some(s!("cba@gmail.com")),
    ..Default::default()
  }];

  for edit_info in edit_infos.iter() {
    let _ = User::update(edit_info, &conn).unwrap();
  }

  let users = User::get_all(&conn).unwrap();
  let first = users.len();

  let _del = User::delete(id, &conn);
  let users = User::get_all(&conn).unwrap();
  assert_eq!(users.len() + 1, first);
}
