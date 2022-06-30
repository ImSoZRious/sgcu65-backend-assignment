use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

use diesel::result::Error;

#[derive(Queryable, Debug, QueryableByName, Clone, Serialize)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

#[derive(Deserialize)]
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

#[derive(Default, Deserialize, Debug)]
pub struct UserQuery {
  pub id: Option<i32>,
  pub email: Option<String>,
  pub firstname: Option<String>,
  pub lastname: Option<String>,
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

  pub fn query(query: &UserQuery, conn: &PgConnection) -> Result<Vec<User>, Error> {
    let filter_string = query.to_filter_string();

    let query_string = format!(
      "SELECT * \
    FROM users \
    WHERE {}",
      filter_string
    );

    diesel::sql_query(query_string).load(conn)
  }

  pub fn query_id(ids: Vec<i32>, conn: &PgConnection) -> Result<Vec<User>, Error> {
    all_users.filter(users::id.eq_any(ids)).load(conn)
  }
}

impl UpdateUser {
  pub fn all_none(&self) -> bool {
    self.email.is_none()
      && self.firstname.is_none()
      && self.lastname.is_none()
      && self.role.is_none()
  }
}

impl UserQuery {
  pub fn all_none(&self) -> bool {
    self.firstname.is_none() && self.lastname.is_none()
  }

  pub fn to_filter_string(&self) -> String {
    let mut filter: Vec<String> = Vec::new();

    if let Some(email) = &self.email {
      filter.push(format!("email LIKE '{}%'", email));
    }
    if let Some(id) = &self.id {
      filter.push(format!("id LIKE '{}%'", id));
    }
    if let Some(firstname) = &self.firstname {
      filter.push(format!("firstname LIKE '{}%'", firstname));
    }
    if let Some(lastname) = &self.lastname {
      filter.push(format!("lastname LIKE '{}%'", lastname));
    }
    if let Some(role) = &self.role {
      filter.push(format!("role LIKE '{}%'", role));
    }

    filter.join(" AND ")
  }
}
