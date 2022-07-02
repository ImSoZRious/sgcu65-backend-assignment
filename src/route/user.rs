use super::super::model::user::{NewUser, PartialUser, User};

use super::{Json, MyError};

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
}

async fn update(
  body: web::Json<PartialUser>,
  path: web::Path<(i32,)>,
  pool: web::Data<Pool>,
) -> impl Responder {
  if body.all_none() {
    return HttpResponse::BadRequest();
  }

  let mut user = body.0;
  let id = path.into_inner().0;
  let conn = pool.get().expect("Cannot get connection");
  user.id = Some(id);

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
}

async fn search(query: web::Query<PartialUser>, pool: web::Data<Pool>) -> impl Responder {
  if query.0.all_none() {
    return Err(MyError::InvalidFormat);
  }

  let parsed_query = PartialUser {
    firstname: query.0.firstname,
    lastname: query.0.lastname,
    ..Default::default()
  };

  let conn = pool.get().expect("Cannot get connection");

  let result = User::query(&parsed_query, &conn).map_err(convert_error);

  result
    .and_then(|user| serde_json::to_string(&user).map_err(convert_error))
    .map(|user_string| Json(user_string))
}

async fn get_team(path: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
  let id = path.into_inner().0;
  let conn = pool.get().expect("Cannot get connection");

  User::find(id, &conn)
    .map_err(|err| match err {
      diesel::result::Error::NotFound => MyError::NotFound,
      _ => MyError::InternalServerError,
    })
    .and_then(|user| {
      let team = user.get_team(&conn).map_err(convert_error)?;
      if team.is_none() {
        return Ok(Json("{}".to_string()));
      }
      let ret = serde_json::to_string(&team).map_err(convert_error);
      Ok(Json(ret.unwrap()))
    })
}

pub fn get_scope() -> Scope {
  web::scope("/user")
    .route("", web::get().to(get))
    .route("", web::post().to(create))
    .route("/search", web::get().to(search))
    .route("/{id}/team", web::get().to(get_team))
    .route("/{id}", web::get().to(find))
    .route("/{id}", web::put().to(update))
    .route("/{id}", web::delete().to(delete))
}
