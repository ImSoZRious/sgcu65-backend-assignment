use std::env;

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

  let _res = User::create(new_user, &conn).unwrap();
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

  let _res = User::update(update_info, &conn).unwrap();
}

#[test]
fn delete_user() {
  let conn = get_db_con();

  let delete_id = 2;

  let _ = User::delete(delete_id, &conn).unwrap();
}

fn find_user() {
  let conn = get_db_con();

  let user_id = 3;

  let _ = User::find(user_id, &conn).unwrap();
}
