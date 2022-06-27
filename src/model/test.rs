use super::User;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[test]
fn connection_test() {
  let database_url = "postgres://postgres:trust@localhost:12345/isd";
  let conn =
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

  let users = User::get_all_users(&conn);
  println!("{:?}", users);
}
