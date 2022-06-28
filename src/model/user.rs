use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, QueryableByName};
use rocket::http::Status;
use serde::{Deserialize, Serialize};

use rocket::data::{self, ByteUnit, Data, FromData};
use rocket::request::Request;

use diesel::result::Error;

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 1024;

#[derive(Queryable, Debug, QueryableByName, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

#[derive(Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct UpdateUser {
  #[serde(default)]
  pub id: i32,
  #[serde(default)]
  pub email: Option<String>,
  #[serde(default)]
  pub firstname: Option<String>,
  #[serde(default)]
  pub lastname: Option<String>,
  #[serde(default)]
  pub role: Option<String>,
}

impl User {
  pub fn create(user: &NewUser, conn: &PgConnection) -> Result<User, Error> {
    diesel::insert_into(users::table)
      .values(user)
      .get_results(conn)
      .map(|user_vec: Vec<User>| user_vec[0].clone())
  }

  pub fn get_all(conn: &PgConnection) -> Result<Vec<User>, Error> {
    all_users.order(users::id.desc()).load::<User>(conn)
  }

  pub fn update(user: &UpdateUser, conn: &PgConnection) -> Result<(), Error> {
    let mut set_string_vec: Vec<String> = vec![];

    if let Some(email) = &user.email {
      set_string_vec.push(format!("email = '{}'", email));
    }
    if let Some(firstname) = &user.firstname {
      set_string_vec.push(format!("firstname = '{}'", firstname));
    }
    if let Some(lastname) = &user.lastname {
      set_string_vec.push(format!("lastname = '{}'", lastname));
    }
    if let Some(role) = &user.role {
      set_string_vec.push(format!("role = '{}'", role));
    }

    let set_string = set_string_vec.join(",");

    let query_string: String = format!(
      "UPDATE users \
    SET {} \
    WHERE id = {}",
      set_string, user.id
    );

    // Result is not actually user since this query isn't select query
    let res: Result<Vec<User>, Error> = diesel::sql_query(query_string).load(conn);

    match res {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
    }
  }

  pub fn delete(user_id: i32, conn: &PgConnection) -> Result<usize, Error> {
    diesel::delete(all_users)
      .filter(users::id.eq(user_id))
      .execute(conn)
  }

  pub fn find(user_id: i32, conn: &PgConnection) -> Result<User, Error> {
    all_users.find(user_id).first(conn)
  }
}
