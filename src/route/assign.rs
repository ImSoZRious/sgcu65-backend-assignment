use super::super::model::assign::UsersTask;

use super::{Json, MyError};

use super::convert_error;

use actix_web::{web, Responder, Scope};

use crate::db::Pool;

async fn assign_task(body: web::Json<UsersTask>, pool: web::Data<Pool>) -> impl Responder {
  let conn = pool.get().expect("Cannot get connection");

  let users_tasks = body.0;

  let result = UsersTask::create(&users_tasks, &conn);

  result
    .map_err(|err| match err {
      diesel::result::Error::DatabaseError(
        diesel::result::DatabaseErrorKind::ForeignKeyViolation,
        _,
      ) => MyError::NotFound,
      _ => MyError::InternalServerError,
    })
    .and_then(|ut| serde_json::to_string(&ut).map_err(convert_error))
    .map(|ut_string| Json(ut_string))
}

pub fn get_scope() -> Scope {
  web::scope("/assign").route("", web::post().to(assign_task))
}
