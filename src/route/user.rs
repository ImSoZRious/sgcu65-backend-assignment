use super::super::model::user::{NewUser, UpdateUser, User};

use super::Json;

use super::convert_error;

use actix_web::{web, HttpResponse, Responder, Scope};

use crate::db::Pool;

async fn get(pool: web::Data<Pool>) -> impl Responder {
  // "resource"
  let conn = pool.get().expect("Cannot get connection");

  let result = User::get_all(&conn).map_err(convert_error);

  result
    .and_then(|user| serde_json::to_string(&user).map_err(convert_error))
    .map(|user_string| Json(user_string))
}

async fn create(body: web::Json<NewUser>, pool: web::Data<Pool>) -> impl Responder {
  let user = body.0;
  let conn = pool.get().expect("Cannot get connection");

  let result = User::create(&user, &conn).map_err(convert_error);

  result
    .and_then(|users| serde_json::to_string(&users).map_err(convert_error))
    .map(|user_string| Json(user_string))
    .map_err(convert_error)
}

async fn update(
  body: web::Json<UpdateUser>,
  path: web::Path<(i32,)>,
  pool: web::Data<Pool>,
) -> impl Responder {
  let mut user = body.0;
  let id = path.into_inner().0;
  let conn = pool.get().expect("Cannot get connection");
  user.id = id;

  let result = User::update(&user, &conn);

  match result {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError(),
  }
}

async fn delete(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");
  let id = path.into_inner().0;
  let result = User::delete(id, &conn);

  match result {
    Ok(1) => HttpResponse::Ok(),
    Ok(_) => HttpResponse::NotFound(),
    Err(_) => HttpResponse::InternalServerError(),
  }
}

async fn find(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let id = path.into_inner().0;

  let result = User::find(id, &conn).map_err(convert_error);

  result
    .and_then(|user| serde_json::to_string(&user).map_err(convert_error))
    .map(|user_string| Json(user_string))
    .map_err(convert_error)
}

pub fn get_scope() -> Scope {
  web::scope("/user")
    .route("", web::get().to(get))
    .route("/{id}", web::get().to(find))
    .route("", web::post().to(create))
    .route("/{id}", web::delete().to(delete))
    .route("/{id}", web::put().to(update))
}
