// this module doesn't count as compilation unit, so it doesn't impact performance
// of binary in production
#[allow(dead_code)]
#[allow(unused_imports)]
use std::env;

mod accept_task;
mod assign_user;
mod task;
mod team;
mod user;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[macro_export]
macro_rules! s {
  ($s: expr) => {{
    $s.to_string()
  }};
}

pub(self) use s;

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
