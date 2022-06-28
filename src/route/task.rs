use super::super::db::DbConn;
use super::super::model::task::{NewTask, Task, UpdateTask};

use super::Json;

use rocket::http::Status;

#[get("/")]
pub async fn get(conn: DbConn) -> Result<Json, Status> {
  let result = Task::get_all(&conn);

  match result {
    Ok(tasks) => match serde_json::to_string(&tasks) {
      Ok(task_string) => Ok(Json(task_string)),
      Err(_) => Err(Status::InternalServerError),
    },
    Err(_) => Err(Status::InternalServerError),
  }
}

#[post("/", format = "application/json", data = "<task>")]
pub async fn create(task: NewTask, conn: DbConn) -> Result<Json, Status> {
  let result = Task::create(&task, &conn);

  match result {
    Ok(tasks) => match serde_json::to_string(&tasks) {
      Ok(task_string) => Ok(Json(task_string)),
      Err(_) => Err(Status::InternalServerError),
    },
    Err(_) => Err(Status::InternalServerError),
  }
}

#[put("/<id>", format = "application/json", data = "<task>")]
pub async fn update(mut task: UpdateTask, conn: DbConn, id: i32) -> Status {
  task.id = id;

  let result = Task::update(&task, &conn);

  match result {
    Ok(_) => Status::Ok,
    Err(_) => Status::InternalServerError,
  }
}

#[delete("/<id>")]
pub async fn delete(id: i32, conn: DbConn) -> Status {
  let result = Task::delete(id, &conn);

  match result {
    Ok(1) => Status::Ok,
    Ok(_) => Status::NotFound,
    Err(_) => Status::InternalServerError,
  }
}
