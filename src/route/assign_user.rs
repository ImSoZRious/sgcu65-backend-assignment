use super::MyError;

use super::super::model::user::User;
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;

use crate::db::Pool;

#[derive(Deserialize)]
struct AssignUser {
  team_id: i32,
  user_id: i32,
}

async fn assign_task(body: web::Json<AssignUser>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let assign_user = body.0;

  let result = User::join_team(assign_user.user_id, assign_user.team_id, &conn);

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
  web::scope("/assign_user").route("", web::post().to(assign_task))
}
