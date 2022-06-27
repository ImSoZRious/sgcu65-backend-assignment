use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

impl User {
  pub fn create_user(user: NewUser, conn: &PgConnection) -> bool {
    diesel::insert_into(users::table)
      .values(&user)
      .execute(conn)
      .is_ok()
  }

  pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
    all_users
      .order(users::id.desc())
      .load::<User>(conn)
      .expect("error!")
  }
}
