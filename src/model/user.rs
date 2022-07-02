use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

use diesel::result::Error;

#[derive(Serialize, Deserialize, Debug, Queryable, QueryableByName, Insertable, Clone)]
#[table_name = "users"]
// struct that is returned from DB
pub struct User {
  pub id: i32,
  pub firstname: String,
  pub lastname: String,
  pub email: String,
  pub role: String,
  pub team_id: Option<i32>,
  #[serde(skip)]
  pub pwd_hash: String,
  pub permission: String,
}

#[derive(Deserialize)]
// Struct that get from user
pub struct NewUser {
  pub firstname: String,
  pub lastname: String,
  pub email: String,
  pub role: String,
  pub pwd: String,
}

// Struct that is converted from NewUser
#[derive(Insertable)]
#[table_name = "users"]
struct InsertUser {
  pub firstname: String,
  pub lastname: String,
  pub email: String,
  pub role: String,
  pub team_id: Option<i32>,
  pub pwd_hash: String,
  pub permission: String,
}

// Struct use for query, update
#[derive(Serialize, Deserialize, Default)]
pub struct PartialUser {
  pub id: Option<i32>,
  pub firstname: Option<String>,
  pub lastname: Option<String>,
  pub email: Option<String>,
  pub role: Option<String>,
  pub team_id: Option<i32>,
  pub pwd_hash: Option<String>,
  pub permission: Option<String>,
}

impl User {
  pub fn get_all(conn: &PgConnection) -> Result<Vec<User>, Error> {
    all_users.order(users::id.desc()).load::<User>(conn)
  }

  pub fn create(user: &NewUser, conn: &PgConnection) -> Result<User, Error> {
    diesel::insert_into(users::table)
      .values::<InsertUser>(user.into())
      .get_results(conn)
      .map(|user_vec: Vec<User>| user_vec[0].clone())
  }

  pub fn update(user: &PartialUser, conn: &PgConnection) -> Result<(), Error> {
    let set_string = user.set_string();

    if user.no_id() {
      panic!("No id is supplied");
    }

    if user.all_none() {
      panic!("all other field are None")
    }

    let query_string: String = format!(
      "UPDATE users \
    SET {} \
    WHERE id = {}",
      set_string,
      user.id.unwrap()
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

  pub fn query(query: &PartialUser, conn: &PgConnection) -> Result<Vec<User>, Error> {
    let filter_string = query.like_string();

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

impl PartialUser {
  pub fn all_none(&self) -> bool {
    self.firstname.is_none()
      && self.lastname.is_none()
      && self.email.is_none()
      && self.role.is_none()
      && self.permission.is_none()
      && self.team_id.is_none()
  }

  pub fn no_id(&self) -> bool {
    self.id.is_none()
  }

  fn join(&self, op: impl ToString, sep: impl ToString) -> String {
    let mut string_vec: Vec<String> = vec![];
    let op = op.to_string();

    if let Some(email) = &self.email {
      string_vec.push(format!("email {} '{}'", op, email));
    }
    if let Some(firstname) = &self.firstname {
      string_vec.push(format!("firstname {} '{}'", op, firstname));
    }
    if let Some(lastname) = &self.lastname {
      string_vec.push(format!("lastname {} '{}'", op, lastname));
    }
    if let Some(role) = &self.role {
      string_vec.push(format!("role {} '{}'", op, role));
    }
    if let Some(permission) = &self.permission {
      string_vec.push(format!("permission {} '{}'", op, permission));
    }
    if let Some(team_id) = &self.team_id {
      string_vec.push(format!("team_id {} '{}'", op, team_id));
    }

    string_vec.join(sep.to_string().as_str())
  }

  pub fn set_string(&self) -> String {
    self.join("=", ",")
  }

  pub fn filter_string(&self) -> String {
    self.join("=", " AND ")
  }

  pub fn like_string(&self) -> String {
    let mut string_vec: Vec<String> = vec![];

    if let Some(email) = &self.email {
      string_vec.push(format!("email LIKE '{}%'", email));
    }
    if let Some(firstname) = &self.firstname {
      string_vec.push(format!("firstname LIKE '{}%'", firstname));
    }
    if let Some(lastname) = &self.lastname {
      string_vec.push(format!("lastname LIKE '{}%'", lastname));
    }
    if let Some(role) = &self.role {
      string_vec.push(format!("role LIKE '{}%'", role));
    }
    if let Some(permission) = &self.permission {
      string_vec.push(format!("permission LIKE '{}%'", permission));
    }
    if let Some(team_id) = &self.team_id {
      string_vec.push(format!("team_id LIKE '{}%'", team_id));
    }

    string_vec.join(" AND ")
  }
}

// temporary hash function
fn hash(pwd: &String) -> String {
  pwd
    .as_bytes()
    .iter()
    .map(|&v| v as i32)
    .sum::<i32>()
    .to_string()
}

impl From<&NewUser> for InsertUser {
  fn from(user: &NewUser) -> Self {
    InsertUser {
      email: user.email.clone(),
      firstname: user.firstname.clone(),
      lastname: user.lastname.clone(),
      permission: "User".to_string(),
      pwd_hash: hash(&user.pwd.clone()),
      role: user.role.clone(),
      team_id: None,
    }
  }
}
