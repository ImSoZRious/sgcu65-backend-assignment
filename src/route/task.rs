use super::super::model::task::{NewTask, Task, UpdateTask};

use super::Json;

use super::convert_error;

use actix_web::{web, HttpResponse, Responder, Scope};

use crate::db::Pool;

async fn get(pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let result = Task::get_all(&conn).map_err(convert_error);

  result
    .and_then(|task| serde_json::to_string(&task).map_err(convert_error))
    .map(|task_string| Json(task_string))
}

async fn create(body: web::Json<NewTask>, pool: web::Data<Pool>) -> impl Responder {
  let task = body.0;
  let conn = pool.get().expect("Cannot get connection");

  let result = Task::create(&task, &conn).map_err(convert_error);

  result
    .and_then(|tasks| serde_json::to_string(&tasks).map_err(convert_error))
    .map(|task_string| Json(task_string))
    .map_err(convert_error)
}

async fn update(
  body: web::Json<UpdateTask>,
  path: web::Path<(i32,)>,
  pool: web::Data<Pool>,
) -> impl Responder {
  let mut task = body.0;
  let id = path.into_inner().0;
  let conn = pool.get().expect("Cannot get connection");
  task.id = id;

  let result = Task::update(&task, &conn);

  match result {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError(),
  }
}

async fn delete(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");
  let id = path.into_inner().0;
  let result = Task::delete(id, &conn);

  match result {
    Ok(1) => HttpResponse::Ok(),
    Ok(_) => HttpResponse::NotFound(),
    Err(_) => HttpResponse::InternalServerError(),
  }
}

async fn find(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let id = path.into_inner().0;

  let result = Task::find(id, &conn).map_err(convert_error);

  result
    .and_then(|task| serde_json::to_string(&task).map_err(convert_error))
    .map(|task_string| Json(task_string))
    .map_err(convert_error)
}

pub fn get_scope() -> Scope {
  web::scope("/task")
    .route("", web::get().to(get))
    .route("/{id}", web::get().to(find))
    .route("", web::post().to(create))
    .route("/{id}", web::delete().to(delete))
    .route("/{id}", web::put().to(update))
}
