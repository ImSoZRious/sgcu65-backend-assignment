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
}
#[derive(Insertable, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
  pub name: String,
  pub content: String,
  pub status: String,
  pub deadline: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
  #[serde(default)]
  pub id: i32,
  #[serde(default)]
  pub name: Option<String>,
  #[serde(default)]
  pub content: Option<String>,
  #[serde(default)]
  pub status: Option<String>,
  #[serde(default)]
  pub deadline: Option<String>,
}

#[derive(Default)]
pub struct TaskQuery<'a> {
  pub name: Option<&'a str>,
}

impl Task {
  pub fn create(task: &NewTask, conn: &PgConnection) -> Result<Task, Error> {
    diesel::insert_into(tasks::table)
      .values(task)
      .get_results(conn)
      .map(|task_vec: Vec<Task>| task_vec[0].clone())
  }

  pub fn get_all(conn: &PgConnection) -> Result<Vec<Task>, Error> {
    all_tasks.order(tasks::id.desc()).load::<Task>(conn)
  }

  pub fn update(task: &UpdateTask, conn: &PgConnection) -> Result<(), Error> {
    let mut set_string_vec: Vec<String> = vec![];

    if let Some(name) = &task.name {
      set_string_vec.push(format!("name = '{}'", name));
    }
    if let Some(content) = &task.content {
      set_string_vec.push(format!("content = '{}'", content));
    }
    if let Some(status) = &task.status {
      set_string_vec.push(format!("status = '{}'", status));
    }
    if let Some(deadline) = &task.deadline {
      set_string_vec.push(format!("deadline = '{}'", deadline));
    }

    let set_string = set_string_vec.join(",");

    let query_string: String = format!(
      "UPDATE tasks \
    SET {} \
    WHERE id = {}",
      set_string, task.id
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

  pub fn query(query: &TaskQuery, conn: &PgConnection) -> Result<Vec<Task>, Error> {
    let mut filter: Vec<String> = Vec::new();
    if let Some(name) = &query.name {
      filter.push(format!("name LIKE '{}%'", name));
    }

    let filter_string = filter.join(", ");

    let query_string = format!(
      "SELECT * \
    FROM tasks \
    WHERE {}",
      filter_string
    );

    diesel::sql_query(query_string).load(conn)
  }
}
