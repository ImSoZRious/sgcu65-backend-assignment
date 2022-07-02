use super::super::model::team::{NewTeam, PartialTeam, Team};

use super::{Json, MyError};

use super::convert_error;

use actix_web::{web, HttpResponse, Responder, Scope};

use crate::db::Pool;

async fn get(pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let result = Team::get_all(&conn).map_err(convert_error);

  result
    .and_then(|team| serde_json::to_string(&team).map_err(convert_error))
    .map(|team_string| Json(team_string))
}

async fn create(body: web::Json<NewTeam>, pool: web::Data<Pool>) -> impl Responder {
  let team = body.0;
  let conn = pool.get().expect("Cannot get connection");

  let result = Team::create(&team, &conn).map_err(convert_error);

  result
    .and_then(|teams| serde_json::to_string(&teams).map_err(convert_error))
    .map(|team_string| Json(team_string))
}

async fn update(
  body: web::Json<PartialTeam>,
  path: web::Path<(i32,)>,
  pool: web::Data<Pool>,
) -> impl Responder {
  if body.all_none() {
    return HttpResponse::BadRequest();
  }

  let mut team = body.0;
  let id = path.into_inner().0;
  let conn = pool.get().expect("Cannot get connection");
  team.id = Some(id);

  let result = Team::update(&team, &conn);

  match result {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError(),
  }
}

async fn delete(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");
  let id = path.into_inner().0;
  let result = Team::delete(id, &conn);

  match result {
    Ok(1) => HttpResponse::Ok(),
    Ok(_) => HttpResponse::NotFound(),
    Err(_) => HttpResponse::InternalServerError(),
  }
}

async fn find(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let id = path.into_inner().0;

  let result = Team::find(id, &conn).map_err(convert_error);

  result
    .and_then(|team| serde_json::to_string(&team).map_err(convert_error))
    .map(|team_string| Json(team_string))
}

async fn search(query: web::Query<PartialTeam>, pool: web::Data<Pool>) -> impl Responder {
  if query.0.all_none() {
    return Err(MyError::InvalidFormat);
  }

  let parsed_query = PartialTeam {
    name: query.0.name,
    ..Default::default()
  };

  let conn = pool.get().expect("Cannot get connection");

  let result = Team::query(&parsed_query, &conn).map_err(convert_error);

  result
    .and_then(|team| serde_json::to_string(&team).map_err(convert_error))
    .map(|team_string| Json(team_string))
}

async fn get_user(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let id = path.into_inner().0;

  let conn = pool.get().expect("Cannot get connection");

  Team::find(id, &conn)
    .map_err(convert_error)
    .and_then(|team| team.get_user(&conn).map_err(convert_error))
    .and_then(|users| serde_json::to_string(&users).map_err(convert_error))
    .map(|user_string| Json(user_string))
}

async fn get_task(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let id = path.into_inner().0;

  let conn = pool.get().expect("Cannot get connection");

  Team::find(id, &conn)
    .map_err(convert_error)
    .and_then(|team| team.get_task(&conn).map_err(convert_error))
    .and_then(|tasks| serde_json::to_string(&tasks).map_err(convert_error))
    .map(|task_string| Json(task_string))
}

pub fn get_scope() -> Scope {
  web::scope("/team")
    .route("", web::get().to(get))
    .route("", web::post().to(create))
    .route("/search", web::get().to(search))
    .route("/{id}/user", web::get().to(get_user))
    .route("/{id}/task", web::get().to(get_task))
    .route("/{id}", web::get().to(find))
    .route("/{id}", web::put().to(update))
    .route("/{id}", web::delete().to(delete))
}
