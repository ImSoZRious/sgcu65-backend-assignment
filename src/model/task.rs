use crate::schema::tasks;
use crate::schema::users::dsl::users as all_users;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct Task {
  pub id: i32,
  pub name: String,
  pub content: String,
  pub status: String,
  pub deadline: String,
}
#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
  pub name: String,
  pub content: String,
  pub status: String,
  pub deadline: String,
}
