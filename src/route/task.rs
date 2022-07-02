use super::super::model::task::{NewTask, PartialTask, Task};

use super::{Json, MyError};

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
}

async fn update(
  body: web::Json<PartialTask>,
  path: web::Path<(i32,)>,
  pool: web::Data<Pool>,
) -> impl Responder {
  if body.all_none() {
    return HttpResponse::BadRequest();
  }

  let mut task = body.0;
  let id = path.into_inner().0;
  let conn = pool.get().expect("Cannot get connection");
  task.id = Some(id);

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
}

async fn search(query: web::Query<PartialTask>, pool: web::Data<Pool>) -> impl Responder {
  if query.0.all_none() {
    return Err(MyError::InvalidFormat);
  }

  let parsed_query = PartialTask {
    name: query.0.name,
    ..Default::default()
  };

  let conn = pool.get().expect("Cannot get connection");

  let result: Result<Vec<Task>, MyError> = Task::query(&parsed_query, &conn).map_err(convert_error);

  result
    .and_then(|task| serde_json::to_string(&task).map_err(convert_error))
    .map(|task_string| Json(task_string))
}

async fn get_team(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let id = path.into_inner().0;
  let conn = pool.get().expect("Cannot get connection");

  Task::find(id, &conn)
    .map_err(|err| match err {
      diesel::result::Error::NotFound => MyError::NotFound,
      _ => MyError::InternalServerError,
    })
    .and_then(|task| {
      let team = task.get_team(&conn).map_err(convert_error)?;
      if team.is_none() {
        return Ok(Json("{}".to_string()));
      }
      let ret = serde_json::to_string(&team).map_err(convert_error);
      Ok(Json(ret.unwrap()))
    })
}

pub fn get_scope() -> Scope {
  web::scope("/task")
    .route("", web::get().to(get))
    .route("", web::post().to(create))
    .route("/search", web::get().to(search))
    .route("/{id}/team", web::get().to(get_team))
    .route("/{id}", web::get().to(find))
    .route("/{id}", web::put().to(update))
    .route("/{id}", web::delete().to(delete))
}
