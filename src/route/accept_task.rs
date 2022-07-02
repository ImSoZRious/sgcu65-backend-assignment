use super::{Json, MyError};

use super::convert_error;

use super::super::model::task::Task;
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;

use crate::db::Pool;

#[derive(Deserialize)]
struct AcceptTask {
  team_id: i32,
  task_id: i32,
}

async fn assign_task(body: web::Json<AcceptTask>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let accept_task = body.0;

  let result = Task::assign_to(accept_task.task_id, accept_task.team_id, &conn);

  result
    .map_err(|err| match err {
      diesel::result::Error::DatabaseError(
        diesel::result::DatabaseErrorKind::ForeignKeyViolation,
        _,
      ) => MyError::NotFound,
      _ => MyError::InternalServerError,
    })
    .map(|_| HttpResponse::Ok())
}

pub fn get_scope() -> Scope {
  web::scope("/accept_task").route("", web::post().to(assign_task))
}
