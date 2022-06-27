use diesel;
use diesel::prelude::*;

#[derive(Serialize, Queryable)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

#[derive(Serialize, Queryable)]
pub struct Task {
  pub id: i32,
  pub name: String,
  pub content: String,
  pub status: String,
  pub deadline: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
  pub email: String,
  pub firstname: String,
  pub lastname: String,
  pub role: String,
}
