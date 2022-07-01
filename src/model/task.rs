use crate::schema::tasks;
use crate::schema::tasks::dsl::tasks as all_tasks;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, QueryableByName, Clone, Serialize)]
#[table_name = "tasks"]
pub struct Task {
  pub id: i32,
  pub name: String,
  pub content: String,
  pub status: String,
  pub deadline: String,
  pub owner_team_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct NewTask {
  pub name: String,
  pub content: String,
  pub status: String,
  pub deadline: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
struct InsertTask {
  pub name: String,
  pub content: String,
  pub status: String,
  pub deadline: String,
  pub owner_team_id: Option<i32>,
}

#[derive(Deserialize, Default)]
pub struct PartialTask {
  pub id: Option<i32>,
  pub name: Option<String>,
  pub content: Option<String>,
  pub status: Option<String>,
  pub deadline: Option<String>,
  pub owner_team_id: Option<i32>,
}

impl Task {
  pub fn create(task: &NewTask, conn: &PgConnection) -> Result<Task, Error> {
    diesel::insert_into(tasks::table)
      .values::<InsertTask>(task.into())
      .get_results(conn)
      .map(|task_vec: Vec<Task>| task_vec[0].clone())
  }

  pub fn get_all(conn: &PgConnection) -> Result<Vec<Task>, Error> {
    all_tasks.order(tasks::id.desc()).load::<Task>(conn)
  }

  pub fn update(task: &PartialTask, conn: &PgConnection) -> Result<(), Error> {
    let set_string = task.set_string();

    if task.no_id() {
      panic!("No id is supplied");
    }

    if task.all_none() {
      panic!("all other field are None")
    }

    let query_string: String = format!(
      "UPDATE tasks \
        SET {} \
        WHERE id = {}",
      set_string,
      task.id.unwrap()
    );

    // Result is not actually task since this query isn't select query
    let res: Result<Vec<Task>, Error> = diesel::sql_query(query_string).load(conn);

    match res {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
    }
  }

  pub fn delete(task_id: i32, conn: &PgConnection) -> Result<usize, Error> {
    diesel::delete(all_tasks)
      .filter(tasks::id.eq(task_id))
      .execute(conn)
  }

  pub fn find(task_id: i32, conn: &PgConnection) -> Result<Task, Error> {
    all_tasks.find(task_id).first(conn)
  }

  pub fn query(query: &PartialTask, conn: &PgConnection) -> Result<Vec<Task>, Error> {
    let filter_string = query.like_string();

    let query_string = format!(
      "SELECT * \
      FROM tasks \
      WHERE {}",
      filter_string
    );

    diesel::sql_query(query_string).load(conn)
  }

  pub fn query_id(ids: Vec<i32>, conn: &PgConnection) -> Result<Vec<Task>, Error> {
    all_tasks.filter(tasks::id.eq_any(ids)).load(conn)
  }
}

impl PartialTask {
  pub fn all_none(&self) -> bool {
    self.name.is_none()
      && self.content.is_none()
      && self.status.is_none()
      && self.deadline.is_none()
      && self.owner_team_id.is_none()
  }

  pub fn no_id(&self) -> bool {
    self.id.is_none()
  }

  fn join(&self, op: impl ToString, sep: impl ToString) -> String {
    let mut string_vec: Vec<String> = vec![];
    let op = op.to_string();

    if let Some(name) = &self.name {
      string_vec.push(format!("name {} '{}'", op, name));
    }
    if let Some(content) = &self.content {
      string_vec.push(format!("content {} '{}'", op, content));
    }
    if let Some(status) = &self.status {
      string_vec.push(format!("status {} '{}'", op, status));
    }
    if let Some(deadline) = &self.deadline {
      string_vec.push(format!("deadline {} '{}'", op, deadline));
    }
    if let Some(owner_team_id) = &self.owner_team_id {
      string_vec.push(format!("owner_team_id {} '{}'", op, owner_team_id));
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

    if let Some(name) = &self.name {
      string_vec.push(format!("name LIKE '{}%'", name));
    }
    if let Some(content) = &self.content {
      string_vec.push(format!("content LIKE '{}%'", content));
    }
    if let Some(status) = &self.status {
      string_vec.push(format!("status LIKE '{}%'", status));
    }
    if let Some(deadline) = &self.deadline {
      string_vec.push(format!("deadline LIKE '{}%'", deadline));
    }
    if let Some(owner_team_id) = &self.owner_team_id {
      string_vec.push(format!("owner_team_id LIKE '{}%'", owner_team_id));
    }

    string_vec.join(" AND ")
  }
}

impl std::convert::From<&NewTask> for InsertTask {
  fn from(task: &NewTask) -> Self {
    InsertTask {
      name: task.name.clone(),
      content: task.content.clone(),
      status: task.status.clone(),
      deadline: task.deadline.clone(),
      owner_team_id: None,
    }
  }
}
