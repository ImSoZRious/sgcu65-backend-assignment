use crate::schema::users_tasks;
use crate::schema::users_tasks::dsl::users_tasks as all_ut;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use diesel::PgConnection;

use super::task::Task;
use super::user::User;

use diesel::result::Error;

#[derive(Clone, Serialize, Insertable, Queryable, Debug, Deserialize)]
pub struct UsersTask {
  pub user_id: i32,
  pub task_id: i32,
}

impl UsersTask {
  pub fn create(assign: &UsersTask, conn: &PgConnection) -> Result<UsersTask, Error> {
    diesel::insert_into(users_tasks::table)
      .values(assign)
      .get_results(conn)
      .map(|assign: Vec<UsersTask>| assign[0].clone())
  }

  pub fn from_user(user_id: i32, conn: &PgConnection) -> Result<Vec<Task>, Error> {
    let res: Result<Vec<UsersTask>, Error> = all_ut
      .select((users_tasks::user_id, users_tasks::task_id))
      .filter(users_tasks::user_id.eq_all(user_id))
      .load(conn);

    res.and_then(|uts: Vec<UsersTask>| {
      Task::query_id(uts.into_iter().map(|ut| ut.task_id).collect(), conn)
    })
  }

  pub fn from_task(task_id: i32, conn: &PgConnection) -> Result<Vec<User>, Error> {
    let res: Result<Vec<UsersTask>, Error> = all_ut
      .select((users_tasks::user_id, users_tasks::task_id))
      .filter(users_tasks::task_id.eq_all(task_id))
      .load(conn);

    res.and_then(|uts: Vec<UsersTask>| {
      User::query_id(uts.into_iter().map(|ut| ut.user_id).collect(), conn)
    })
  }
}
